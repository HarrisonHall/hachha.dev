use axum::{extract::Path, extract::State, response::Html};
use log::*;
use rust_embed::RustEmbed;
use serde_json::json;

use crate::site::SharedSite;
use crate::util;

#[derive(RustEmbed)]
#[folder = "content/pages/projects"]
pub struct ProjectsFiles;

pub async fn visit_projects<'a>(State(site): State<SharedSite<'a>>) -> Html<String> {
    let projects_page = util::read_embedded_text::<ProjectsFiles>("projects.html").unwrap();
    return Html(site.render_page(&projects_page, &json!({})));
}
