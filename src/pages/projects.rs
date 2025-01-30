//! Projects.

use super::*;

/// Raw embedded project files.
#[derive(RustEmbed)]
#[folder = "content/pages/projects"]
struct EmbeddedProjectsFiles;

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

/// Parsed project data.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Project {
    /// Title of the blog.
    name: String,
    /// Description of blog.
    description: String,
    /// Date.
    date: String,
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
            date: "".to_string(),
            image: None,
            links: HashMap::new(),
        }
    }
}

#[derive(Clone)]
pub struct ProjectPage {
    index: String,
    projects: Projects,
}

impl ProjectPage {
    pub fn new() -> Result<Self> {
        let index = util::read_embedded_text::<EmbeddedProjectsFiles>("projects.html")?;
        let projects =
            util::read_embedded_toml::<Projects, EmbeddedProjectsFiles>("projects.toml")?;
        Ok(ProjectPage { index, projects })
    }

    pub fn project_metadata(&self) -> serde_json::Value {
        return json!({
            "projects": self.projects.iter().map(|proj| proj.to_json()).collect::<Vec<serde_json::Value>>()
        });
    }
}

pub async fn visit_projects(State(site): State<SharedSite>) -> RenderedHtml {
    match site.page_cache().retrieve("projects").await {
        Ok(page) => page,
        Err(_) => {
            let proj_metadata = site.pages().projects.project_metadata();
            site.page_cache()
                .update(
                    "projects",
                    site.render_page(&site.pages().projects.index, &proj_metadata),
                )
                .await
        }
    }
}
