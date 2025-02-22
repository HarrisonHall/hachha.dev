//! Links page.

use super::*;

use atom_syndication as atom;

/// The links page and subpages.
pub struct LinksPage {
    index: String,
    #[allow(unused)]
    links: Links,
    feed: String,
    metadata: serde_json::Value,
}

impl LinksPage {
    /// Generate new links pages.
    pub fn new() -> Result<Self> {
        // Parse pages.
        let index = util::read_embedded_text::<EmbeddedLinksFiles>("links.html")?;
        let mut links = util::read_embedded_toml::<Links, EmbeddedLinksFiles>("links.toml")?;
        links.sort();
        links.reverse();

        // Parse into atom feed.
        let mut feed_builder = atom::FeedBuilder::default();
        feed_builder
            .title("hachha.dev Links")
            .author(atom::PersonBuilder::default().name("Harrison Hall").build())
            .link(
                atom::LinkBuilder::default()
                    .href("https://hachha.dev")
                    .title("hachha.dev".to_string())
                    .build(),
            )
            .icon("https://hachha.dev/media/catman.png".to_string())
            .subtitle(Some("hachha.dev link feed".to_string().into()));
        let mut entries = Vec::new();
        for link in links.iter() {
            let timestamp: chrono::DateTime<chrono::FixedOffset> = link
                .date
                .and_time(chrono::NaiveTime::default())
                .and_local_timezone(chrono::Utc)
                .latest()
                .ok_or(anyhow!("Unable to convert dt {} to utc.", link.date))?
                .into();
            let entry: atom::Entry = atom::EntryBuilder::default()
                .title(link.title.clone())
                .summary(Some(link.description.clone().into()))
                .link(
                    atom::LinkBuilder::default()
                        .href(link.url.clone())
                        .title(link.title.clone())
                        .build(),
                )
                .published(timestamp)
                .updated(timestamp)
                .build();
            entries.push(entry);
        }
        feed_builder.entries(entries);
        let feed = feed_builder.build().to_string();

        // Generate metadata. Darken every other entry.
        let mut metadata = json!({});
        let mut link_metadata: Vec<serde_json::Value> = Vec::new();
        for (i, link) in links.iter().enumerate() {
            let mut meta = util::to_json(link)?;
            util::merge_json(&mut meta, &json!({"darken": i % 2 == 0}))?;
            link_metadata.push(meta);
        }
        metadata["links"] = serde_json::Value::Array(link_metadata);

        Ok(LinksPage {
            index,
            links,
            feed,
            metadata,
        })
    }
}

/// Parsed link list.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Links {
    /// Links.
    links: Vec<Link>,
}

impl Default for Links {
    fn default() -> Self {
        Self { links: Vec::new() }
    }
}

impl std::ops::Deref for Links {
    type Target = Vec<Link>;
    fn deref(&self) -> &Self::Target {
        &self.links
    }
}

impl std::ops::DerefMut for Links {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.links
    }
}

/// Embedded links files.
#[derive(RustEmbed)]
#[folder = "content/pages/blog/links"]
struct EmbeddedLinksFiles;

/// Parsed link data.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Link {
    /// Title of the link/article.
    title: String,
    /// Short description of the link.
    description: String,
    /// URL or link.
    url: String,
    /// Date link was added.
    date: chrono::NaiveDate,
}

impl Default for Link {
    fn default() -> Self {
        Link {
            title: "".into(),
            description: "".into(),
            url: "".into(),
            date: chrono::NaiveDate::default(),
        }
    }
}

impl std::cmp::PartialOrd for Link {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

impl std::cmp::Ord for Link {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

/// Endpoint for links index page.
pub async fn visit_links_index(State(site): State<Site>) -> RenderedHtml {
    match site.page_cache().retrieve("links").await {
        Ok(page) => page,
        Err(_) => {
            site.page_cache()
                .update(
                    "links",
                    site.render_page(&site.pages().links.index, &site.pages().links.metadata),
                )
                .await
        }
    }
}

/// Get links as atom feed.
pub async fn visit_links_feed(State(site): State<Site>) -> impl axum::response::IntoResponse {
    (
        [(axum::http::header::CONTENT_TYPE, "application/atom+xml")],
        site.pages().links.feed.clone(),
    )
}
