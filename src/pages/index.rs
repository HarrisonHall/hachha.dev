//! Index (home) page.

use super::*;

/// Raw index page.
#[derive(RustEmbed)]
#[folder = "content/pages/index"]
struct EmbeddedIndexPage;

/// The index (home) page.
pub struct IndexPage {
    /// Unrendered page.
    raw_page: String,
    /// Json context for rendering page.
    config: Config,
}

impl IndexPage {
    /// Generate new index page.
    pub fn new() -> Result<Self> {
        Ok(IndexPage {
            raw_page: util::read_embedded_text::<EmbeddedIndexPage>("index.html")?,
            config: util::read_embedded_toml::<Config, EmbeddedIndexPage>("config.toml")?,
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
    match site.page_cache().retrieve("index").await {
        Ok(page) => page,
        Err(_) => {
            let context = json!({
                "phrase": site.pages().index.get_phrase(),
            });
            site.page_cache()
                .update(
                    "index",
                    site.render_page(&site.pages().index.raw_page, &context),
                )
                .await
        }
    }
}
