//! Index (home) page.

use super::*;

/// The index (home) page.
pub struct IndexPage {
    /// Unrendered page.
    raw_page: String,
    /// Json context for rendering page.
    config: Config,
}

impl IndexPage {
    /// Generate new index page.
    pub fn new(packed_data: Arc<PackedData>) -> Result<Self> {
        Ok(IndexPage {
            raw_page: read_embedded_text::<EmbeddedPages>("index.html")?,
            config: packed_data.read_toml::<Config>("content/index_phrases.toml")?,
        })
    }

    fn get_phrase(&self) -> String {
        self.config
            .phrases
            .choose(&mut rand::rng())
            .unwrap_or(&"Uh oh!".to_owned())
            .clone()
    }
}

/// Configuration for the index page.
#[derive(Clone, Serialize, Deserialize)]
struct Config {
    /// Phrases that can appear on the index page.
    phrases: Vec<String>,
    /// Highlighted links.
    links: Vec<Link>,
}

/// Link on the index page.
#[derive(Clone, Serialize, Deserialize)]
struct Link {
    /// Name of the link.
    name: String,
    /// Url.
    url: String,
    /// Target field for
    #[serde(default)]
    target: Option<String>,
}

/// Endpoint for site index.
pub async fn visit_index(uri: Uri, State(site): State<Site>) -> RenderedHtml {
    EndpointHistoryOptions::default()
        .write(&site, uri.path())
        .await;
    site.clone()
        .page_cache()
        .retrieve_or_update("index", async move {
            let context = json!({
                "phrase": site.pages().index.get_phrase(),
                "links": site.pages().index.config.links.clone(),
            });
            site.render_page(&site.pages().index.raw_page, &context)
        })
        .await
}
