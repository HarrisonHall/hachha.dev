//! Index (home) page.

use super::*;

/// Meta pages struct.
pub struct MetaPages;

impl MetaPages {
    /// Generate new index page.
    pub fn new(_packed_data: Arc<PackedData>) -> Result<Self> {
        Ok(MetaPages)
    }
}

#[derive(Serialize, Deserialize)]
pub struct VersionData {
    version: String,
    commit_hash: String,
    build_time: String,
}

impl Default for VersionData {
    fn default() -> Self {
        let version = match env!("CARGO_PKG_VERSION") {
            "" => "0.0.1",
            v => v,
        };
        let commit_hash = match env!("COMMIT_HASH") {
            "" => "<unknown>",
            h => h,
        };
        let build_time = match env!("BUILD_TIME") {
            "" => "<unknown>",
            h => h,
        };

        Self {
            version: version.into(),
            commit_hash: commit_hash.into(),
            build_time: build_time.into(),
        }
    }
}

/// Endpoint for site version.
pub async fn version(uri: Uri, State(site): State<Site>) -> axum::Json<VersionData> {
    EndpointHistoryOptions::default()
        .write(&site, uri.path())
        .await;
    VersionData::default().into()
}
