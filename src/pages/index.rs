use axum::{extract::State, response::Html};

use rust_embed::RustEmbed;
use serde_json::json;

use crate::site::SharedSite;
use crate::util;

#[derive(RustEmbed)]
#[folder = "content/pages/"]
#[exclude = "*/*"]
#[include = "index.html"]
struct EmbeddedIndexPage;

pub struct IndexPage {
    pub raw_page: String,
    pub index_context: serde_json::Value,
}

impl IndexPage {
    pub fn new() -> Self {
        IndexPage {
            raw_page: util::read_embedded_text::<EmbeddedIndexPage>("index.html").unwrap(),
            index_context: json!({}),
        }
    }
}

pub async fn visit_index<'a>(State(site): State<SharedSite<'a>>) -> Html<String> {
    match site.page_cache.retrieve("index") {
        Ok(page) => page,
        Err(_) => site.page_cache.update(
            "index",
            Html(site.render_page(&site.pages.index.raw_page, &json!({}))),
        ),
    }
}
