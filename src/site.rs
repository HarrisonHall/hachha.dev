use std::path::PathBuf;

use clap::Parser;
use handlebars::Handlebars;
use log::*;
use rust_embed::RustEmbed;


pub struct Site<'a> {
    pub config: SiteConfig,
    pub templater: Handlebars<'a>,
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
        }
    }
}


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct SiteConfig {
    #[arg(short, long, value_name = "PORT", default_value_t = 8080)]
    pub port: u16,
    #[arg(short, long, value_name = "LOG_PATH")]
    pub log: Option<String>,
    #[arg(short, long, value_name = "DIRECTORY")]
    pub content: Option<PathBuf>,
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}


#[derive(RustEmbed)]
#[folder = "content/templates/"]
pub struct Templates;

fn create_templater<'a>() -> Handlebars<'a> {
    let mut templater = Handlebars::new();
    for item in Templates::iter() {
        let raw_template = Templates::get(&item).unwrap();
        let template = std::str::from_utf8(&raw_template.data).unwrap();
        let template_name: String = item.to_string(); //.strip_prefix("templates/").unwrap().to_owned();
        let res = templater.register_partial(&format!("templates/{}", template_name), template);
        if res.is_err() {
            error!("Unable to register partial {}", template_name);
        }
    }
    return templater;
}