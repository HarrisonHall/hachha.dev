//! Site helper.

use super::*;

use clap::Parser;
use handlebars::Handlebars;

/// Shared type representing site.
#[derive(Clone)]
pub struct SharedSite(Arc<Site>);

impl SharedSite {
    /// Generate shared site.
    pub fn new() -> Result<Self> {
        Ok(SharedSite(Arc::new(Site::new()?)))
    }

    /// Get config.
    pub fn config(&self) -> &SiteConfig {
        &self.0.config
    }

    /// Get templater.
    pub fn templater(&self) -> &Handlebars {
        &self.0.templater
    }

    /// Get pages.
    pub fn pages(&self) -> &Pages {
        &self.0.pages
    }

    /// Get page cache.
    pub fn page_cache(&self) -> &Cache<RenderedHtml> {
        &self.0.page_cache
    }

    /// Get content cache.
    pub fn content_cache(&self) -> &Cache<EmbeddedData> {
        &self.0.content_cache
    }

    /// Generate base context.
    pub fn base_context(&self) -> serde_json::Value {
        let current_time = chrono::Utc::now();
        return json!({
            "version": env!("CARGO_PKG_VERSION"),
            "year": current_time.year(),
        });
    }

    /// Render page with templater given json values.
    pub fn render_page(&self, page: &String, metadata: &serde_json::Value) -> RenderedHtml {
        // Compute complete json to render page.
        let mut render_context = self.base_context();
        if util::merge_json(&mut render_context, metadata).is_err() {
            error!("Unable to merge json to render page.");
            return RenderedHtml::new(pages::error::WORST_CASE_404);
        };

        RenderedHtml::new(
            match self.templater().render_template(&page, &render_context) {
                Ok(rendered_page) => rendered_page,
                Err(e) => {
                    error!("Error rendering page: {e}");
                    pages::error::WORST_CASE_404.to_string()
                }
            },
        )
    }
}

/// Site struct.
pub struct Site {
    pub config: SiteConfig,
    pub templater: Arc<Handlebars<'static>>,
    pub pages: Pages,
    pub page_cache: Cache<RenderedHtml>,
    pub content_cache: Cache<EmbeddedData>,
}

impl Site {
    /// Generate new site object.
    pub fn new() -> Result<Self> {
        // Parse arguments
        let args: SiteConfig = SiteConfig::parse();

        // Set logging
        std::env::set_var("RUST_LOG", "info");
        if args.debug {
            std::env::set_var("RUST_LOG", "debug");
        }
        pretty_env_logger::init();

        // Configure site struct
        Ok(Site {
            templater: Arc::new(create_templater()?),
            pages: Pages::new()?,
            page_cache: Cache::new(args.cache_timeout),
            content_cache: Cache::new(args.cache_timeout),
            config: args,
        })
    }
}

/// CLI arguments for server.
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

fn create_templater<'a>() -> Result<Handlebars<'a>> {
    let mut templater = Handlebars::new();

    // Register templates.
    for item in Templates::iter() {
        let raw_template = match Templates::get(&item) {
            Some(raw) => raw,
            None => bail!("Unabel to get template for templater: {item:?}"),
        };
        let template = std::str::from_utf8(&raw_template.data)?;
        let template_name: String = item.to_string(); //.strip_prefix("templates/").expect("...").to_owned();
        let res = templater.register_partial(&format!("templates/{}", template_name), template);
        if res.is_err() {
            error!("Unable to register partial {}: {:?}", template_name, res);
        }
    }

    // Register helpers.
    templater.register_helper("markdown", Box::new(util::markdown_helper));

    Ok(templater)
}
