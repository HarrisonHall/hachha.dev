//! Database wrapper.

use super::*;

/// Database helper.
/// This is utilized to track persistent state and history within the site.
pub struct Database {
    db: turso::Database,
}

impl Database {
    /// Create a new database handle.
    pub async fn new(fname: impl AsRef<str>) -> Result<Self> {
        let sqlite_db = match turso::Builder::new_local(fname.as_ref()).build().await {
            Ok(sqlite_db) => sqlite_db,
            Err(e) => {
                bail!("Failed to initialize database: {e}");
            }
        };

        let db = Self { db: sqlite_db };
        db.initialize().await?;

        Ok(db)
    }

    /// Initialize the database.
    async fn initialize(&self) -> Result<()> {
        let version = self
            .database_version()
            .await
            .unwrap_or_else(|_| semver::Version::new(0, 0, 0));

        let conn = self.db.connect()?;
        if version < semver::Version::new(0, 0, 1) {
            conn.execute(
                "
                -- Table for version tracking.
                CREATE TABLE IF NOT EXISTS version_history(
                    id INTEGER PRIMARY KEY ASC,
                    -- Semver version.
                    version TEXT NOT NULL,
                    -- The timestamp of the upgrade.
                    timestamp INTEGER NOT NULL
                ) STRICT;
                ",
                (),
            )
            .await?;
            conn.execute(
                "
                -- Table for tracking endpoint history.
                CREATE TABLE IF NOT EXISTS endpoint_history(
                    id INTEGER PRIMARY KEY ASC,
                    -- Semver version.
                    endpoint TEXT NOT NULL UNIQUE,
                    -- The timestamp of the first attempt.
                    first_timestamp INTEGER NOT NULL,
                    -- The timestamp of the last attempt.
                    last_timestamp INTEGER NOT NULL,
                    -- Number of times the timestamp was hit.
                    count INTEGER NOT NULL DEFAULT 0,
                    -- Whether or not the endpoint is valid.
                    valid INTEGER NOT NULL DEFAULT 0
                ) STRICT;
                ",
                (),
            )
            .await?;
            conn.execute(
                "
                INSERT INTO
                    version_history(version, timestamp)
                    VALUES(?, unixepoch(date('now')));
                ",
                [semver::Version::new(0, 0, 1).to_string().as_str()],
            )
            .await?;
        }

        Ok(())
    }

    /// Get database version.
    async fn database_version(&self) -> Result<semver::Version> {
        // NOTE: Could check for table.
        // let res = sqlx::query(
        //     "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='version_history';"
        // );

        let conn = self.db.connect()?;
        let mut version_rows = conn
            .query(
                "SELECT version FROM version_history ORDER BY id DESC LIMIT 1",
                (),
            )
            .await?;
        let row = version_rows.next().await?;
        if row.is_none() {
            bail!("No versions in version_history table.");
        }
        let row = row.unwrap();
        let version_string: String = row.get(0)?;

        match semver::Version::parse(&version_string) {
            Ok(v) => Ok(v),
            Err(e) => {
                bail!("Failed to parse version_history semver: {e}");
            }
        }
    }

    /// Add entry to endpoint history.
    pub async fn update_endpoint_history(
        &self,
        endpoint: impl AsRef<str>,
        options: EndpointHistoryOptions,
    ) -> Result<()> {
        let conn = self.db.connect()?;
        conn.busy_timeout(std::time::Duration::from_millis(50)).ok();
        let res = conn.execute(
            concat!(
                "INSERT INTO endpoint_history(endpoint, count, valid, first_timestamp, last_timestamp) ",
                "VALUES(?, ?, ?, unixepoch(date('now')), unixepoch(date('now'))) ",
                "ON CONFLICT (endpoint) ",
                "DO UPDATE SET ",
                "last_timestamp = unixepoch(date('now')), count = count + 1, valid = excluded.valid",
            ),
            (endpoint.as_ref(), 1, if options.valid { 1 } else { 0 },),
        )
        .await;

        if let Err(e) = res {
            tracing::warn!("Failed to write to endpoint history: {e}");
            bail!(e);
        }

        Ok(())
    }
}

#[derive(Builder, Clone, Copy)]
#[derive(Default)]
pub struct EndpointHistoryOptions {
    pub valid: bool,
}

impl EndpointHistoryOptions {
    pub async fn write(&self, site: &Site, endpoint: impl AsRef<str>) {
        site.db()
            .update_endpoint_history(endpoint, *self)
            .await
            .ok();
    }
}

