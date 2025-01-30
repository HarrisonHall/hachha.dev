//! Site helper.

use super::*;

use clap::Parser;
use handlebars::Handlebars;

/// Shareable site state wrapper.
#[derive(Clone)]
pub struct Site(Arc<SiteWrapped>);

impl Site {
    /// Generate shared site.
    pub fn new() -> Result<Self> {
        Ok(Site(Arc::new(SiteWrapped::new()?)))
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

    /// Generate base context.
    pub fn base_context(&self) -> serde_json::Value {
        let current_time = chrono::Utc::now();
        return json!({
            "version": env!("CARGO_PKG_VERSION"),
            "year": current_time.year(),
        });
    }

    /// Render page with templater given json values.
    pub fn render_page(&self, page: impl AsRef<str>, metadata: &serde_json::Value) -> RenderedHtml {
        // Compute complete json to render page.
        let mut render_context = self.base_context();
        if util::merge_json(&mut render_context, metadata).is_err() {
            error!("Unable to merge json to render page.");
            return RenderedHtml::new(pages::error::WORST_CASE_404);
        };

        RenderedHtml::new(
            match self
                .templater()
                .render_template(page.as_ref(), &render_context)
            {
                Ok(rendered_page) => rendered_page,
                Err(e) => {
                    error!("Error rendering page: {e}");
                    pages::error::WORST_CASE_404.to_string()
                }
            },
        )
    }
}

/// Site state.
struct SiteWrapped {
    config: SiteConfig,
    templater: Arc<Handlebars<'static>>,
    pages: Pages,
    page_cache: Cache<RenderedHtml>,
}

impl SiteWrapped {
    /// Generate new site object.
    fn new() -> Result<Self> {
        // Parse arguments
        let args: SiteConfig = SiteConfig::parse();

        // Set logging
        std::env::set_var("RUST_LOG", "info");
        if args.debug {
            std::env::set_var("RUST_LOG", "debug");
        }
        pretty_env_logger::init();

        // Configure site struct
        Ok(SiteWrapped {
            templater: Arc::new(create_templater()?),
            pages: Pages::new()?,
            page_cache: Cache::new(args.cache_timeout),
            config: args,
        })
    }
}

/// CLI arguments for server.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct SiteConfig {
    /// Port to serve on.
    #[arg(short, long, value_name = "PORT", default_value_t = 8443)]
    pub port: u16,
    /// Log file path.
    #[arg(long, value_name = "LOG_PATH")]
    pub log: Option<String>,
    /// Timeout for cache (seconds).
    #[arg(long, default_value_t = 5.0 * 60.0)]
    pub cache_timeout: f32,
    /// Debug logging.
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}

#[derive(RustEmbed)]
#[folder = "content/templates/"]
pub struct Templates;

/// Create handlebars templater for the site.
fn create_templater<'a>() -> Result<Handlebars<'a>> {
    use handlebars::handlebars_helper;
    use markdown;

    static MD_RENDERING_ERROR: &str = "<p>Unable to render markdown :(</p>";

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

    handlebars_helper!(markdown_helper: |content: String| {
        let mut md_options = markdown::Options::gfm();
        md_options.compile.allow_dangerous_html = true;
        md_options.compile.allow_dangerous_protocol = true;
        let mut compiled_markdown = match markdown::to_html_with_options(&content, &md_options) {
            Ok(compiled_markdown) => compiled_markdown,
            Err(e) => {
                error!("Markdown rendering error: {}", e);
                MD_RENDERING_ERROR.to_string()
            }
        };
        compiled_markdown = compiled_markdown.replace("<pre>", "<pre class=\"code\">");  // Replace code blocks
        compiled_markdown
    });

    // Register helpers.
    templater.register_helper("markdown", Box::new(markdown_helper));

    Ok(templater)
}
