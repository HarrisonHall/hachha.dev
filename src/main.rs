use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::Path,
    extract::State,
    routing::{get, post, get_service},
    response::Html,
    Json, Router
};
use log::*;
use rust_embed::RustEmbed;
use serde_json::json;

mod site;
mod pages;
mod util;

use crate::site::Site;


#[tokio::main]
async fn main() {
    // Generate site data
    let site: Arc<Site> = Arc::new(Site::new());

    // Set up routing
    let mut app = Router::new();
    app = app.route("/", get(pages::visit_index));
    app = app.route("/styles/*path", get(style));
    app = app.route("/fonts/*path", get(font));
    app = app.route("/blog", get(pages::visit_blog_index));
    app = app.route("/blog/", get(pages::visit_blog_index));
    app = app.route("/blog/*path", get(pages::visit_blog));
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
#[folder = "content/styles/"]
pub struct Styles;
async fn style(Path(path): Path<String>) -> Vec<u8> {
    // TODO - decide if we should return text/css mime type
    match Styles::get(&path) {
        Some(file) => {
           //std::str::from_utf8(&(file.data)).unwrap().to_string()
           file.data.to_vec()
        },
        None => {
            error!("Asked for invalid asset at style/{}", path);
            //"".to_owned()
            vec![]
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