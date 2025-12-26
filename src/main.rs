//! Main.

use std::borrow::Cow;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Once;
use std::time::Instant;

use anyhow::{anyhow, bail, Result};
use axum::extract::{Path, State};
use axum::response::Html;
use axum::{routing::get, Router};
use chrono::Datelike;
use rand::seq::IndexedRandom;
use rust_embed::RustEmbed;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use tokio::sync::RwLock;

mod cache;
mod pages;
mod resources;
mod site;
mod theme;
mod util;

use crate::cache::Cache;
use crate::pages::Pages;
use crate::site::Site;
use crate::theme::ThemeProvider;
use crate::util::*;

/// Server entry-point.
#[tokio::main]
async fn main() -> Result<()> {
    // Build/parse site and serve.
    let site = Site::new()?;
    site.serve().await?;
    Ok(())
}
