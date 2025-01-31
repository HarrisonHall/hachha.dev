//! Projects.

use super::*;

/// Raw embedded project files.
#[derive(RustEmbed)]
#[folder = "content/pages/projects"]
struct EmbeddedProjectsFiles;

/// The project page.
#[derive(Clone)]
pub struct ProjectsPage {
    /// Unrendered index page.
    raw_index: String,
    /// Project list.
    projects: Projects,
    /// All projects metadata.
    metadata: serde_json::Value,
}

impl ProjectsPage {
    /// Generate new projects page.
    pub fn new() -> Result<Self> {
        // Parse index file.
        let raw_index = util::read_embedded_text::<EmbeddedProjectsFiles>("projects.html")?;

        // Parse projects.
        let mut projects =
            util::read_embedded_toml::<Projects, EmbeddedProjectsFiles>("projects.toml")?;
        projects.sort();
        projects.reverse();

        // Compile metadata from projects.
        let metadata = json!({
            "projects": projects.iter().map(|proj| proj.to_json()).collect::<Vec<serde_json::Value>>()
        });

        // Build projects page.
        Ok(ProjectsPage {
            raw_index,
            projects,
            metadata,
        })
    }
}

/// Parsed project list.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Projects {
    projects: Vec<Project>,
}

impl std::ops::Deref for Projects {
    type Target = Vec<Project>;
    fn deref(&self) -> &Self::Target {
        &self.projects
    }
}

impl std::ops::DerefMut for Projects {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.projects
    }
}

/// Parsed project data.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Project {
    /// Title of the blog.
    name: String,
    /// Description of blog.
    description: String,
    /// Date.
    date: chrono::NaiveDate,
    /// Path to image (if existent).
    image: Option<String>,
    /// Links to project.
    links: HashMap<String, String>,
}

impl Project {
    /// Convert project data to json.
    fn to_json(&self) -> serde_json::Value {
        json!({
            "name": self.name,
            "description": self.description,
            "date": self.date,
            "image": self.image,
            "links": self.links,
        })
    }
}

impl Default for Project {
    fn default() -> Self {
        Project {
            name: "".to_string(),
            description: "".to_string(),
            date: chrono::NaiveDate::default(),
            image: None,
            links: HashMap::new(),
        }
    }
}

impl std::cmp::PartialOrd for Project {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

impl std::cmp::Ord for Project {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

/// Endpoint for project index page.
pub async fn visit_projects(State(site): State<Site>) -> RenderedHtml {
    match site.page_cache().retrieve("projects").await {
        Ok(page) => page,
        Err(_) => {
            site.page_cache()
                .update(
                    "projects",
                    site.render_page(
                        &site.pages().projects.raw_index,
                        &site.pages().projects.metadata,
                    ),
                )
                .await
        }
    }
}
