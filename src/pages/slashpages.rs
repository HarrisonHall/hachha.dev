//! /slashpages.

use super::*;

/// The index (home) page.
pub struct SlashPages {
    /// Unrendered page.
    raw_page: String,
    /// All pages (markdown).
    pages: HashMap<String, String>,
}

impl SlashPages {
    /// Generate new index page.
    pub fn new(packed_data: Arc<PackedData>) -> Result<Self> {
        let mut sp = Self {
            raw_page: read_embedded_text::<EmbeddedPages>("slashpages/template.html")?,
            pages: HashMap::new(),
        };

        for (full_path, _data) in packed_data.iter() {
            if !full_path.starts_with("pages/slashpages/pages/") {
                continue;
            }

            let path = full_path
                .replace("pages/slashpages/pages/", "")
                .replace(".md", "");

            if path.starts_with("_") {
                continue;
            }

            if path.trim().len() == 0 {
                continue;
            }

            if let Ok(data) = packed_data.read_text(&full_path) {
                if data.len() == 0 {
                    continue;
                }

                sp.pages.insert(path, data);
            }
        }

        Ok(sp)
    }

    /// Add slashpages.
    pub fn add_routes(&self, mut router: Router<Site>) -> Router<Site> {
        for slashpage in self.pages.keys() {
            tracing::debug!("Adding slashpage /{}", slashpage);
            router = router.route(&format!("/{slashpage}"), get(visit_slashpage));
        }

        router
    }
}

/// Endpoint for slashpages.
pub async fn visit_slashpage(
    uri: Uri,
    State(site): State<Site>,
    headers: HeaderMap,
) -> RenderedHtml {
    let slashpage = match uri.path().strip_prefix("/") {
        Some(t) => t,
        None => uri.path(),
    }
    .to_owned();
    if !site.pages().slashpages.pages.contains_key(&slashpage) {
        tracing::error!("Invalid slashpage: {}", slashpage);
        return error::visit_404_internal(slashpage, State(site), Some(headers)).await;
    }

    let page_name = slashpage.clone();
    site.clone()
        .page_cache()
        .retrieve_or_update(&slashpage, async move {
            let mut content = String::new();
            if let Some(markdown) = site.pages().slashpages.pages.get(&page_name) {
                content = markdown.clone();
            };
            let context = json!({
                "slashpage": page_name,
                "slashpage-content": content,
            });
            site.render_page(&site.pages().slashpages.raw_page, &context)
        })
        .await
}
