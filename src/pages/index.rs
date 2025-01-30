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
    /// Unrendered page.
    raw_page: String,
    /// Json context for rendering page.
    context: serde_json::Value,
}

impl IndexPage {
    /// Generate new index page.
    pub fn new() -> Result<Self> {
        Ok(IndexPage {
            raw_page: util::read_embedded_text::<EmbeddedIndexPage>("index.html")?,
            context: json!({}),
        })
    }
}

/// Endpoint for site index.
pub async fn visit_index(State(site): State<Site>) -> RenderedHtml {
    match site.page_cache().retrieve("index").await {
        Ok(page) => page,
        Err(_) => {
            site.page_cache()
                .update(
                    "index",
                    site.render_page(&site.pages().index.raw_page, &site.pages().index.context),
                )
                .await
        }
    }
}
