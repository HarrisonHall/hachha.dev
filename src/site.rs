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

    /// Serve site.
    pub async fn serve(&self) -> Result<()> {
        // Set up routing.
        let mut app = Router::new();
        app = app.route("/", get(pages::index::visit_index));
        app = app.route("/styles/{*path}", get(resources::get_style));
        app = app.route("/theme.css", get(theme::get_theme));
        app = app.route("/fonts/{*path}", get(resources::get_font));
        app = app.route("/media/{*path}", get(resources::get_media));
        app = app.route("/blog", get(pages::blog::visit_blog_index));
        app = app.route("/blog.feed", get(pages::blog::visit_blog_feed));
        app = app.route("/blog/{:path}", get(pages::blog::visit_blog));
        app = app.route(
            "/blog/{:path}/{*resource}",
            get(pages::blog::get_blog_resource),
        );
        app = app.route("/links", get(pages::links::visit_links_index));
        app = app.route("/links.feed", get(pages::links::visit_links_feed));
        app = app.route("/projects", get(pages::projects::visit_projects));
        app = app.route("/favicon.ico", get(resources::get_favicon));
        app = app.route("/robots.txt", get(resources::get_robots_txt));
        app = app.fallback(get(pages::error::visit_404));
        app = app.layer(tower_http::trace::TraceLayer::new_for_http());

        // Add self as state.
        let app = app.with_state(self.clone());

        // Serve.
        tracing::info!("Serving haccha.dev on {}", self.config().port);
        tracing::debug!("Debug @ http://127.0.0.1:{}", self.config().port);
        let addr = SocketAddr::from(([0, 0, 0, 0], self.config().port));
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app.into_make_service()).await?;

        Ok(())
    }

    /// Get config.
    pub fn config(&self) -> &SiteConfig {
        &self.0.config
    }

    /// Get pages.
    pub fn pages(&self) -> Arc<Pages> {
        match cfg!(debug_assertions) {
            true => match pages::Pages::new() {
                Ok(pages) => Arc::new(pages),
                Err(e) => {
                    tracing::error!("Unable to rebuild pages!");
                    panic!("{e}");
                }
            },
            false => self.0.pages.clone(),
        }
    }

    /// Get theme provider.
    pub fn theme_provider(&self) -> Arc<ThemeProvider> {
        self.0.theme_provider.clone()
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
            tracing::error!("Unable to merge json to render page.");
            return RenderedHtml::new(pages::error::WORST_CASE_404);
        };

        RenderedHtml::new(
            match self
                .0
                .templater
                .render_template(page.as_ref(), &render_context)
            {
                Ok(rendered_page) => rendered_page,
                Err(e) => {
                    tracing::error!("Error rendering page: {e}");
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
    pages: Arc<Pages>,
    theme_provider: Arc<ThemeProvider>,
    page_cache: Cache<RenderedHtml>,
}

impl SiteWrapped {
    /// Generate new site object.
    fn new() -> Result<Self> {
        // Parse arguments
        let args: SiteConfig = SiteConfig::parse();

        // Set logging
        color_eyre::install()?;
        util::init_logging(args.debug)?;

        // Configure site struct
        Ok(SiteWrapped {
            templater: Arc::new(create_templater()?),
            pages: Arc::new(Pages::new()?),
            theme_provider: Arc::new(ThemeProvider::new()?),
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
            tracing::error!("Unable to register partial {}: {:?}", template_name, res);
        }
    }

    // Compile markdown to html.
    handlebars_helper!(markdown_helper: |content: String| {
        let mut md_options = markdown::Options::gfm();
        md_options.compile.allow_dangerous_html = true;
        md_options.compile.allow_dangerous_protocol = true;
        let mut compiled_markdown = match markdown::to_html_with_options(&content, &md_options) {
            Ok(compiled_markdown) => compiled_markdown,
            Err(e) => {
                tracing::error!("Markdown rendering error: {}", e);
                MD_RENDERING_ERROR.to_string()
            }
        };
        compiled_markdown = compiled_markdown.replace("<pre>", "<pre class=\"code\">");  // Replace code blocks
        compiled_markdown
    });

    // Allow html to be rendered normally (not escaped)-- dangerous, if used incorrectly.
    handlebars_helper!(html_helper: |content: String| {
        content
    });

    // Register helpers.
    templater.register_helper("markdown", Box::new(markdown_helper));
    templater.register_helper("html", Box::new(html_helper));

    Ok(templater)
}
