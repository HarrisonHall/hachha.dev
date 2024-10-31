use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use log::*;

mod cache;
mod pages;
mod resources;
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
        true => Some({
            let cert = cert_dir.join("cert.pem");
            let priv_key = cert_dir.join("privkey.pem");
            // Create config
            let config = RustlsConfig::from_pem_file(cert.clone(), priv_key.clone())
                .await
                .unwrap();
            // Spawn a task to reload tls
            tokio::spawn(site::reload_tls(config.clone(), cert, priv_key));
            config
        }),
        false => None,
    };

    // Set up routing
    let mut app = Router::new();
    app = app.route("/", get(pages::visit_index));
    app = app.route("/styles/*path", get(resources::get_style));
    app = app.route("/fonts/*path", get(resources::get_font));
    app = app.route("/media/*path", get(resources::get_media));
    app = app.route("/blog", get(pages::visit_blog_index));
    app = app.route("/blog.feed", get(pages::visit_blog_feed));
    app = app.route("/blog/:path", get(pages::visit_blog));
    app = app.route("/blog/:path/*resource", get(pages::get_blog_resource));
    app = app.route("/projects", get(pages::visit_projects));
    app = app.route("/favicon.ico", get(resources::get_favicon));
    app = app.fallback(get(pages::visit_404));
    let app = app.with_state(site.clone());

    // Serve
    info!("Serving haccha.dev on {}", site.config.port);
    debug!("Debug @ http://127.0.0.1:{}", site.config.port);
    let addr = SocketAddr::from(([0, 0, 0, 0], site.config.port));
    match tls_config {
        Some(config) => {
            debug!("Serving HTTPS");
            axum_server::bind_rustls(addr, config)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        None => {
            debug!("Serving HTTP");
            axum_server::bind(addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
    };
}
