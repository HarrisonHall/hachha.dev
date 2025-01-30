//! Site page definitions.

use super::*;

pub mod blog;
pub mod error;
pub mod index;
pub mod projects;

/// All pages helper.
pub struct Pages {
    pub index: index::IndexPage,
    pub error: error::ErrorPage,
    pub blog_indexer: blog::BlogsPages,
    pub projects: projects::ProjectsPage,
}

impl Pages {
    /// Generate helper for all pages.
    pub fn new() -> Result<Self> {
        Ok(Pages {
            index: index::IndexPage::new()?,
            error: error::ErrorPage::new()?,
            blog_indexer: blog::BlogsPages::new()?,
            projects: projects::ProjectsPage::new()?,
        })
    }
}
