use std::path::PathBuf;

use chrono::Datelike;
use clap::Parser;
use handlebars::Handlebars;
use log::*;
use rust_embed::RustEmbed;
use serde_json::json;

use crate::pages::BlogIndexer;
use crate::util;

pub struct Site<'a> {
    pub config: SiteConfig,
    pub templater: Handlebars<'a>,
    pub blog_indexer: BlogIndexer,
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
            config: args,
            templater: create_templater(),
            blog_indexer: BlogIndexer::new(),
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
                // TODO 404 page
                "".to_owned()
            }
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct SiteConfig {
    #[arg(short, long, value_name = "PORT", default_value_t = 443)]
    pub port: u16,
    #[arg(short, long, value_name = "CERT_DIR", default_value_t = String::from("/etc/letsencrypt/live/hachha.dev"))]
    pub cert_dir: String,
    #[arg(short, long, value_name = "LOG_PATH")]
    pub log: Option<String>,
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
