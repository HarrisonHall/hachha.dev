use std::net::SocketAddr;
use std::sync::Arc;
use std::path::PathBuf;

use axum::{
    extract::Path,
    extract::State,
    routing::{get, post, get_service},
    response::Html,
    Json, Router
};
use axum_server::tls_rustls::RustlsConfig;
use log::*;
use rust_embed::RustEmbed;

mod site;
mod pages;
mod util;

use crate::site::Site;


#[tokio::main]
async fn main() {
    // Generate site data
    let site: Arc<Site> = Arc::new(Site::new());

    // Try to set up TLS/HTTPS
    let cert_dir = PathBuf::from(&site.config.cert_dir);
    let tls_config: Option<RustlsConfig> = match cert_dir.is_dir() {
        true => {
            Some(RustlsConfig::from_pem_file(
                cert_dir.join("cert.pem"),
                cert_dir.join("privkey.pem"),
            )
            .await
            .unwrap())
        },
        false => {
            None
        }
    };

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
    match tls_config {
        Some(tls_config) => {
            debug!("Serving HTTPS");
            axum_server::bind_rustls(addr, tls_config)
                .serve(app.into_make_service())
                .await
                .unwrap();
        },
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
#[folder = "content/styles/"]
pub struct Styles;
async fn style(Path(path): Path<String>) -> Vec<u8> {
    // TODO - decide if we should return text/css mime type
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
    // TODO - decide if we should return text/? mime type
    match util::read_embedded_data::<Fonts>(&path) {
        Ok(data) => data,
        Err(e) => {
            error!("Asked for invalid asset at fonts/{path}: {e}");
            vec![]
        }
    }
}