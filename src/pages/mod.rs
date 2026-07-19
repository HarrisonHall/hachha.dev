//! Site page definitions.

use super::*;

pub mod blog;
pub mod error;
pub mod index;
pub mod links;
pub mod meta;
pub mod projects;
pub mod slashpages;

/// All pages helper.
pub struct Pages {
    pub index: index::IndexPage,
    pub error: error::ErrorPage,
    pub blogs: blog::BlogsPages,
    pub links: links::LinksPage,
    pub projects: projects::ProjectsPage,
    pub slashpages: slashpages::SlashPages,
    #[allow(unused)]
    pub meta: meta::MetaPages,
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
            slashpages: slashpages::SlashPages::new(packed_data.clone())?,
            meta: meta::MetaPages::new(packed_data.clone())?,
        })
    }
}

/// Embedded page templates.
#[derive(RustEmbed)]
#[folder = "resources/pages"]
#[include = "*.html"]
struct EmbeddedPages;
