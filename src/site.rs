//! Site helper.

use super::*;

use axum_server::tls_rustls::RustlsConfig;
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
    pub fn page_cache(&self) -> &Cache<Html<String>> {
        &self.0.page_cache
    }

    /// Get content cache.
    pub fn content_cache(&self) -> &Cache<Vec<u8>> {
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

    pub fn render_page(&self, page: &String, metadata: &serde_json::Value) -> String {
        let mut render_context = self.base_context();
        util::merge_json(&mut render_context, metadata);

        match self.templater().render_template(&page, &render_context) {
            Ok(rendered_page) => rendered_page,
            Err(e) => {
                error!("Error rendering page: {e}");
                return crate::pages::error::WORST_CASE_404.to_owned();
            }
        }
    }
}

/// Site struct.
pub struct Site {
    pub config: SiteConfig,
    pub templater: Arc<Handlebars<'static>>,
    pub pages: Pages,
    pub page_cache: Cache<Html<String>>,
    pub content_cache: Cache<Vec<u8>>,
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
            templater: Arc::new(create_templater()),
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

fn create_templater<'a>() -> Handlebars<'a> {
    let mut templater = Handlebars::new();

    // Register templates.
    for item in Templates::iter() {
        let raw_template = Templates::get(&item).expect("Unable to get item from templates!");
        let template =
            std::str::from_utf8(&raw_template.data).expect("Unable to convert template.");
        let template_name: String = item.to_string(); //.strip_prefix("templates/").expect("...").to_owned();
        let res = templater.register_partial(&format!("templates/{}", template_name), template);
        if res.is_err() {
            error!("Unable to register partial {}: {:?}", template_name, res);
        }
    }

    // Register helpers.
    templater.register_helper("markdown", Box::new(util::markdown_helper));

    return templater;
}

/// Reload site by reloading config.
pub async fn reload_tls(config: RustlsConfig, cert: PathBuf, priv_key: PathBuf) {
    loop {
        tokio::time::sleep(Duration::from_secs(48 * 60 * 60)).await; // Every 48 hours
        config
            .reload_from_pem_file(cert.clone(), priv_key.clone())
            .await
            .expect("Unable to reload TLS from pem file!");
        debug!("Reloaded rustls configuration");
    }
}
