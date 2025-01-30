//! Main.

use std::borrow::Cow;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use anyhow::{anyhow, bail, Result};
use axum::extract::{Path, State};
use axum::response::Html;
use axum::{routing::get, Router};
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
use crate::util::EmbeddedData;
use crate::util::RenderedHtml;

/// Server entry-point.
#[tokio::main]
async fn main() -> Result<()> {
    // Generate site data.
    let site = SharedSite::new()?;

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
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
