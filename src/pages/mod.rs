//! Site page definitions.

use super::*;

pub mod blog;
pub mod error;
pub mod index;
pub mod projects;

/// Struct of all pages.
pub struct Pages {
    pub index: index::IndexPage,
    pub error: error::ErrorPage,
    pub blog_indexer: blog::BlogIndexer,
    pub projects: projects::ProjectPage,
}

impl Pages {
    /// Generate page struct.
    pub fn new() -> Result<Self> {
        Ok(Pages {
            index: index::IndexPage::new(),
            error: error::ErrorPage::new(),
            blog_indexer: blog::BlogIndexer::new(),
            projects: projects::ProjectPage::new()?,
        })
    }
}
