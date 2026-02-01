//! Error pages.

use axum::http::Uri;

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
pub async fn visit_404(uri: Uri, State(site): State<Site>) -> RenderedHtml {
    visit_404_internal(uri.path(), State(site)).await
}

/// Internal 404
pub async fn visit_404_internal(path: impl AsRef<str>, State(site): State<Site>) -> RenderedHtml {
    tracing::warn!("404: Visit invalid uri `{}`.", path.as_ref());
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
