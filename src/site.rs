use std::sync::Arc;

use axum::response::Html;
use chrono::Datelike;
use clap::Parser;
use handlebars::Handlebars;
use log::*;
use rust_embed::RustEmbed;
use serde_json::json;

use crate::cache::Cache;
use crate::pages::Pages;
use crate::util;

pub type SharedSite<'a> = Arc<Site<'a>>;

pub struct Site<'a> {
    pub config: SiteConfig,
    pub templater: Handlebars<'a>,
    pub pages: Pages,
    pub page_cache: Cache<Html<String>>,
    pub content_cache: Cache<Vec<u8>>,
}

impl<'a> Site<'a> {
    pub fn new() -> Self {
        // Parse arguments
        let args: SiteConfig = SiteConfig::parse();

        // Set logging
        std::env::set_var("RUST_LOG", "info");
        if args.debug {
            std::env::set_var("RUST_LOG", "debug");
        }
        pretty_env_logger::init();

        // Configure site struct
        Site {
            templater: create_templater(),
            pages: Pages::new(),
            page_cache: Cache::new(args.cache_timeout),
            content_cache: Cache::new(args.cache_timeout),
            config: args,
        }
    }

    pub fn get_base_context(&self) -> serde_json::Value {
        let current_time = chrono::Utc::now();
        return json!({
            "version": env!("CARGO_PKG_VERSION"),
            "year": current_time.year(),
        });
    }

    pub fn render_page(&self, page: &String, metadata: &serde_json::Value) -> String {
        let mut render_context = self.get_base_context();
        util::merge_json(&mut render_context, metadata);

        match self.templater.render_template(&page, &render_context) {
            Ok(rendered_page) => rendered_page,
            Err(e) => {
                error!("Error rendering page: {e}");
                return crate::pages::WORST_CASE_404.to_owned();
            }
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct SiteConfig {
    #[arg(short, long, value_name = "PORT", default_value_t = 443)]
    pub port: u16,
    #[arg(long, value_name = "CERT_DIR", default_value_t = String::from("/etc/letsencrypt/live/hachha.dev"))]
    pub cert_dir: String,
    #[arg(long, value_name = "LOG_PATH")]
    pub log: Option<String>,
    #[arg(long, default_value_t = 5.0 * 60.0)] // Default cache every 5 minutes
    pub cache_timeout: f32,
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}

#[derive(RustEmbed)]
#[folder = "content/templates/"]
pub struct Templates;

fn create_templater<'a>() -> Handlebars<'a> {
    let mut templater = Handlebars::new();

    // Register templates
    for item in Templates::iter() {
        let raw_template = Templates::get(&item).unwrap();
        let template = std::str::from_utf8(&raw_template.data).unwrap();
        let template_name: String = item.to_string(); //.strip_prefix("templates/").unwrap().to_owned();
        let res = templater.register_partial(&format!("templates/{}", template_name), template);
        if res.is_err() {
            error!("Unable to register partial {}: {:?}", template_name, res);
        }
    }

    // Register helpers
    templater.register_helper("markdown", Box::new(util::markdown_helper));

    return templater;
}
