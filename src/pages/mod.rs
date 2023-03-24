mod blog;
mod error;
mod index;
mod projects;

pub use blog::*;
pub use error::*;
pub use index::*;
pub use projects::*;

pub struct Pages {
    pub index: IndexPage,
    pub error: ErrorPage,
    pub blog_indexer: BlogIndexer,
    pub projects: ProjectPage,
}

impl Pages {
    pub fn new() -> Self {
        Pages {
            index: IndexPage::new(),
            error: ErrorPage::new(),
            blog_indexer: BlogIndexer::new(),
            projects: ProjectPage::new(),
        }
    }
}
