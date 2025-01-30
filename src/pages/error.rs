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
    pub raw_page: String,
    pub error_context: serde_json::Value,
}

impl ErrorPage {
    /// Generate new error page.
    pub fn new() -> Self {
        ErrorPage {
            raw_page: util::read_embedded_text::<EmbeddedErrorPage>("404.html")
                .expect("Must have 404.html page!"),
            error_context: json!({}),
        }
    }
}

/// Endpoint for visiting the error 404 page.
pub async fn visit_404(State(site): State<SharedSite>) -> Html<String> {
    match site.page_cache().retrieve("404").await {
        Ok(page) => page,
        Err(_) => {
            site.page_cache()
                .update(
                    "404",
                    Html(site.render_page(&site.pages().error.raw_page, &json!({}))),
                )
                .await
        }
    }
}
