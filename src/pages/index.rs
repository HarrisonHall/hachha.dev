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
    pub fn new() -> Self {
        IndexPage {
            raw_page: util::read_embedded_text::<EmbeddedIndexPage>("index.html")
                .expect("Must have index.html page!"),
            index_context: json!({}),
        }
    }
}

pub async fn visit_index(State(site): State<SharedSite>) -> Html<String> {
    match site.page_cache().retrieve("index").await {
        Ok(page) => page,
        Err(_) => {
            site.page_cache()
                .update(
                    "index",
                    Html(site.render_page(&site.pages().index.raw_page, &json!({}))),
                )
                .await
        }
    }
}
