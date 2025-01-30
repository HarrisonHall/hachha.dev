//! Index (home) page.

use super::*;

/// Raw index page.
#[derive(RustEmbed)]
#[folder = "content/pages/"]
#[exclude = "*/*"]
#[include = "index.html"]
struct EmbeddedIndexPage;

/// The index (home) page.
pub struct IndexPage {
    pub raw_page: String,
    pub index_context: serde_json::Value,
}

impl IndexPage {
    /// Generate new index page.
    pub fn new() -> Result<Self> {
        Ok(IndexPage {
            raw_page: util::read_embedded_text::<EmbeddedIndexPage>("index.html")?,
            index_context: json!({}),
        })
    }
}

pub async fn visit_index(State(site): State<SharedSite>) -> RenderedHtml {
    match site.page_cache().retrieve("index").await {
        Ok(page) => page,
        Err(_) => {
            site.page_cache()
                .update(
                    "index",
                    site.render_page(&site.pages().index.raw_page, &json!({})),
                )
                .await
        }
    }
}
