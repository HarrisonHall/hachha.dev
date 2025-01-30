//! Error pages.

use super::*;

pub const WORST_CASE_404: &str = "<html>404</html>";

/// Raw embedded error page.
#[derive(RustEmbed)]
#[folder = "content/pages/"]
#[exclude = "*/*"]
#[include = "404.html"]
struct EmbeddedErrorPage;

/// Error page.
pub struct ErrorPage {
    /// Unrendered page.
    raw_page: String,
    /// Json context for rendering page.
    context: serde_json::Value,
}

impl ErrorPage {
    /// Generate new error page.
    pub fn new() -> Result<Self> {
        Ok(ErrorPage {
            raw_page: util::read_embedded_text::<EmbeddedErrorPage>("404.html")?,
            context: json!({}),
        })
    }
}

/// Endpoint for error 404 page.
pub async fn visit_404(State(site): State<Site>) -> RenderedHtml {
    match site.page_cache().retrieve("404").await {
        Ok(page) => page,
        Err(_) => {
            site.page_cache()
                .update(
                    "404",
                    site.render_page(&site.pages().error.raw_page, &site.pages().error.context),
                )
                .await
        }
    }
}
