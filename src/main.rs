use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::{
    extract::Path,
    routing::{get, get_service, post},
    Json, Router,
};
use axum_server::tls_rustls::RustlsConfig;
use log::*;
use rust_embed::RustEmbed;

mod cache;
mod pages;
mod site;
mod util;

use crate::site::Site;

#[tokio::main]
async fn main() {
    // Generate site data
    let site: Arc<Site> = Arc::new(Site::new());

    // Try to set up TLS/HTTPS
    let cert_dir = PathBuf::from(&site.config.cert_dir);
    let tls_config: Option<RustlsConfig> = match cert_dir.is_dir() {
        true => Some(
            RustlsConfig::from_pem_file(cert_dir.join("cert.pem"), cert_dir.join("privkey.pem"))
                .await
                .unwrap(),
        ),
        false => None,
    };

    // Set up routing
    let mut app = Router::new();
    app = app.route("/", get(pages::visit_index));
    app = app.route("/styles/*path", get(style));
    app = app.route("/fonts/*path", get(font));
    app = app.route("/media/*path", get(get_media));
    app = app.route("/blog", get(pages::visit_blog_index));
    app = app.route("/blog/", get(pages::visit_blog_index));
    app = app.route("/blog/:path", get(pages::visit_blog));
    app = app.route("/blog/:path/*resource", get(pages::get_blog_resource));
    app = app.route("/projects", get(pages::visit_projects));
    app = app.route("/favicon.ico", get(get_favicon));
    app = app.fallback(get(pages::visit_404));
    let app = app.with_state(site.clone());

    // Serve
    info!("Serving haccha.dev on {}", site.config.port);
    let addr = SocketAddr::from(([0, 0, 0, 0], site.config.port));
    match tls_config {
        Some(tls_config) => {
            debug!("Serving HTTPS");
            axum_server::bind_rustls(addr, tls_config)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        None => {
            debug!("Serving HTTP");
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    };
}

#[derive(RustEmbed)]
#[folder = "content/media"]
#[exclude = "*/*"]
#[include = "favicon.ico"]
pub struct Favicon;
async fn get_favicon() -> Vec<u8> {
    match util::read_embedded_data::<Favicon>("favicon.ico") {
        Ok(data) => data,
        Err(_) => {
            error!("Favicon missing!");
            vec![]
        }
    }
}

#[derive(RustEmbed)]
#[folder = "content/media"]
#[exclude = "favicon.ico"]
pub struct Media;
async fn get_media(Path(path): Path<String>) -> Vec<u8> {
    match util::read_embedded_data::<Media>(&path) {
        Ok(data) => data,
        Err(_) => {
            error!("Asked for missing media {path}");
            vec![]
        }
    }
}

#[derive(RustEmbed)]
#[folder = "content/styles/"]
pub struct Styles;
async fn style(Path(path): Path<String>) -> Vec<u8> {
    match util::read_embedded_data::<Styles>(&path) {
        Ok(data) => data,
        Err(e) => {
            error!("Asked for invalid asset at style/{path}: {e}");
            vec![]
        }
    }
}

#[derive(RustEmbed)]
#[folder = "content/fonts/"]
pub struct Fonts;
async fn font(Path(path): Path<String>) -> Vec<u8> {
    match util::read_embedded_data::<Fonts>(&path) {
        Ok(data) => data,
        Err(e) => {
            error!("Asked for invalid asset at fonts/{path}: {e}");
            vec![]
        }
    }
}
