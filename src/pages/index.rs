use std::path::PathBuf;
use std::sync::Arc;

use axum::{
    extract::Path,
    extract::State,
    response::Html,
    routing::{get, get_service, post},
    Json, Router,
};
use clap::Parser;
use handlebars::Handlebars;
use log::*;
use rust_embed::RustEmbed;
use serde_json::json;

use crate::site::Site;

#[derive(RustEmbed)]
#[folder = "content/pages/"]
#[exclude = "*/*"]
#[include = "index.html"]
pub struct IndexPage;

pub async fn visit_index<'a>(State(site): State<Arc<Site<'a>>>) -> Html<String> {
    let index_page = std::str::from_utf8(&IndexPage::get("index.html").unwrap().data)
        .unwrap()
        .to_string(); // TODO avoid unwraps here
    return Html(site.render_page(&index_page, &json!({})));
}
