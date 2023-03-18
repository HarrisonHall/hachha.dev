use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use handlebars::Handlebars;
use log::*;
use rust_embed::RustEmbed;


use axum::{
    extract::Path,
    extract::State,
    routing::{get, post, get_service},
    response::Html,
    Json, Router
};
use serde_json::json;
use chrono::{Datelike};

use crate::site::Site;


#[derive(RustEmbed)]
#[folder = "content/pages/"]
#[exclude = "*/*"]
#[include = "index.html"]
pub struct IndexPage;

pub async fn visit_index<'a>(State(site): State<Arc<Site<'a>>>) -> Html<String> {
    let current_time = chrono::Utc::now();
    return Html(site.templater.render_template(
        std::str::from_utf8(&IndexPage::get("index.html").unwrap().data).unwrap(),
        &json!({
            "version": env!("CARGO_PKG_VERSION"),
            "year": current_time.year(),
        })
    ).unwrap());
}