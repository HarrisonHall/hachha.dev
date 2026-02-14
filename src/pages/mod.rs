//! Site page definitions.

use super::*;

pub mod blog;
pub mod error;
pub mod index;
pub mod links;
pub mod projects;

/// All pages helper.
pub struct Pages {
    pub index: index::IndexPage,
    pub error: error::ErrorPage,
    pub blogs: blog::BlogsPages,
    pub links: links::LinksPage,
    pub projects: projects::ProjectsPage,
}

impl Pages {
    /// Generate helper for all pages.
    pub fn new(packed_data: Arc<PackedData>) -> Result<Self> {
        Ok(Pages {
            index: index::IndexPage::new(packed_data.clone())?,
            error: error::ErrorPage::new(packed_data.clone())?,
            blogs: blog::BlogsPages::new(packed_data.clone())?,
            links: links::LinksPage::new(packed_data.clone())?,
            projects: projects::ProjectsPage::new(packed_data.clone())?,
        })
    }
}

/// Embedded page templates.
#[derive(RustEmbed)]
#[folder = "content/pages"]
#[include = "*.html"]
struct EmbeddedPages;
