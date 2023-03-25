use axum::{extract::State, response::Html};
use rust_embed::RustEmbed;
use serde_json::json;

use crate::site::SharedSite;
use crate::util;

#[derive(RustEmbed)]
#[folder = "content/pages/"]
#[exclude = "*/*"]
#[include = "404.html"]
struct EmbeddedErrorPage;

pub struct ErrorPage {
    pub raw_page: String,
    pub error_context: serde_json::Value,
}

impl ErrorPage {
    pub fn new() -> Self {
        ErrorPage {
            raw_page: util::read_embedded_text::<EmbeddedErrorPage>("404.html").unwrap(),
            error_context: json!({}),
        }
    }
}

pub async fn visit_404<'a>(State(site): State<SharedSite<'a>>) -> Html<String> {
    match site.page_cache.retrieve("404") {
        Ok(page) => page,
        Err(_) => site.page_cache.update(
            "404",
            Html(site.render_page(&site.pages.error.raw_page, &json!({}))),
        ),
    }
}
