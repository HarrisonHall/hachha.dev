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
            raw_page: read_embedded_text::<EmbeddedPages>("index/index.html")?,
            config: packed_data.read_toml::<Config>("pages/index/config.toml")?,
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

#[derive(Clone, Serialize, Deserialize)]
struct Config {
    phrases: Vec<String>,
}

/// Endpoint for site index.
pub async fn visit_index(State(site): State<Site>) -> RenderedHtml {
    site.clone()
        .page_cache()
        .retrieve_or_update("index", async move {
            let context = json!({
                "phrase": site.pages().index.get_phrase(),
            });
            site.render_page(&site.pages().index.raw_page, &context)
        })
        .await
}
