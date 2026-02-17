//! Error pages.

use super::*;

pub const WORST_CASE_404: &str = "<html>404</html>";

/// Error page.
pub struct ErrorPage {
    /// Unrendered page.
    raw_page: String,
    /// Json context for rendering page.
    context: serde_json::Value,
}

impl ErrorPage {
    /// Generate new error page.
    pub fn new(_packed_data: Arc<PackedData>) -> Result<Self> {
        Ok(ErrorPage {
            raw_page: util::read_embedded_text::<EmbeddedPages>("404.html")?,
            context: json!({}),
        })
    }
}

/// Endpoint for error 404 page.
pub async fn visit_404(uri: Uri, State(site): State<Site>, headers: HeaderMap) -> RenderedHtml {
    visit_404_internal(uri.path(), State(site), Some(headers)).await
}

/// Internal 404
pub async fn visit_404_internal(
    path: impl AsRef<str>,
    State(site): State<Site>,
    headers: Option<HeaderMap>,
) -> RenderedHtml {
    // tracing::warn!("HEADERS: {:?}", headers);
    let remote_ip = match &headers {
        Some(headers) => match headers.get("x-forwarded-for") {
            Some(value) => match value.to_str() {
                Ok(value) => value,
                Err(_) => "<Invalid>",
            },
            None => "<Missing-Header>",
        },
        None => "<No-Headers>",
    };
    tracing::warn!(
        "404: Visit invalid uri `{}` from `{}`.",
        path.as_ref(),
        remote_ip
    );
    site.clone()
        .page_cache()
        .retrieve_or_update("404", async move {
            site.render_page(&site.pages().error.raw_page, &site.pages().error.context)
        })
        .await
}
