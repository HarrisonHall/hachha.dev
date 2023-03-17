use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::Path,
    extract::State,
    routing::{get, post, get_service},
    response::Html,
    Json, Router
};
use chrono::{Datelike};
use log::*;
use rust_embed::RustEmbed;
use serde_json::json;

mod site;

use crate::site::Site;


#[tokio::main]
async fn main() {
    // Generate site data
    let site = Arc::new(Site::new());

    // Set up routing
    let mut app = Router::new();
    app = app.route("/", get(index));
    app = app.route("/styles/*path", get(style));
    app = app.route("/fonts/*path", get(font));
    let app = app.with_state(site.clone());
    
    // Serve
    info!("Serving haccha.dev on {}", site.config.port);
    let addr = SocketAddr::from(([0, 0, 0, 0], site.config.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


#[derive(RustEmbed)]
#[folder = "content/pages/"]
pub struct Pages;


async fn index<'a>(State(site): State<Arc<Site<'a>>>) -> Html<String> {
    let current_time = chrono::Utc::now();
    return Html(site.templater.render_template(
        std::str::from_utf8(&Pages::get("index.html").unwrap().data).unwrap(),
        &json!({
            "version": env!("CARGO_PKG_VERSION"),
            "year": current_time.year(),
        })
    ).unwrap());
}


#[derive(RustEmbed)]
#[folder = "content/styles/"]
pub struct Styles;
async fn style(Path(path): Path<String>) -> String {
    // TODO - decide if we should return text/css mime type
    match Styles::get(&path) {
        Some(file) => {
           std::str::from_utf8(&(file.data)).unwrap().to_string()
        },
        None => {
            error!("Asked for invalid asset at style/{}", path);
            "".to_owned()
        }
    }
}

#[derive(RustEmbed)]
#[folder = "content/fonts/"]
pub struct Fonts;
async fn font(Path(path): Path<String>) -> Vec<u8> {
    // TODO - decide if we should return text/? mime type
    match Fonts::get(&path) {
        Some(file) => {
           file.data.to_vec()
        },
        None => {
            error!("Asked for invalid asset at fonts/{}", path);
            vec![]
        }
    }
}