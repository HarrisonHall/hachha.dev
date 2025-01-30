//! Main.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;

use anyhow::{anyhow, bail, Result};
use axum::extract::{Path, State};
use axum::response::Html;
use axum::{routing::get, Router};
use axum_server::tls_rustls::RustlsConfig;
use chrono::Datelike;
use log::*;
use rust_embed::RustEmbed;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use tokio::sync::RwLock;

mod cache;
mod pages;
mod resources;
mod site;
mod util;

use crate::cache::Cache;
use crate::pages::Pages;
use crate::site::SharedSite;

/// Server entry-point.
#[tokio::main]
async fn main() -> Result<()> {
    // Generate site data.
    let site = SharedSite::new()?;

    // Try to set up TLS/HTTPS.
    let cert_dir = PathBuf::from(&site.config().cert_dir);
    let tls_config: Option<RustlsConfig> = match cert_dir.is_dir() {
        true => Some({
            let cert = cert_dir.join("cert.pem");
            let priv_key = cert_dir.join("privkey.pem");
            // Create config.
            let config = RustlsConfig::from_pem_file(cert.clone(), priv_key.clone())
                .await
                .expect("TLS config is invalid.");
            // Spawn a task to reload tls.
            tokio::spawn(site::reload_tls(config.clone(), cert, priv_key));
            config
        }),
        false => None,
    };

    // Set up routing.
    let mut app = Router::new();
    app = app.route("/", get(pages::index::visit_index));
    app = app.route("/styles/*path", get(resources::get_style));
    app = app.route("/fonts/*path", get(resources::get_font));
    app = app.route("/media/*path", get(resources::get_media));
    app = app.route("/blog", get(pages::blog::visit_blog_index));
    app = app.route("/blog.feed", get(pages::blog::visit_blog_feed));
    app = app.route("/blog/:path", get(pages::blog::visit_blog));
    app = app.route("/blog/:path/*resource", get(pages::blog::get_blog_resource));
    app = app.route("/projects", get(pages::projects::visit_projects));
    app = app.route("/favicon.ico", get(resources::get_favicon));
    app = app.fallback(get(pages::error::visit_404));
    let app = app.with_state(site.clone());

    // Serve.
    info!("Serving haccha.dev on {}", site.config().port);
    debug!("Debug @ http://127.0.0.1:{}", site.config().port);
    let addr = SocketAddr::from(([0, 0, 0, 0], site.config().port));
    match tls_config {
        Some(config) => {
            debug!("Serving HTTPS");
            axum_server::bind_rustls(addr, config)
                .serve(app.into_make_service())
                .await
                .expect("Unable to bind for TLS.");
        }
        None => {
            debug!("Serving HTTP");
            axum_server::bind(addr)
                .serve(app.into_make_service())
                .await
                .expect("Unable to serve.");
        }
    };

    Ok(())
}
