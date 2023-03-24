use axum::{extract::Path, extract::State, response::Html};
use log::*;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use crate::site::SharedSite;
use crate::util;

#[derive(RustEmbed)]
#[folder = "content/pages/projects"]
pub struct EmbeddedProjectsFiles;

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct ProjectData {
    /// Title of the blog
    name: String,
    /// Description of blog
    description: String,
    /// Date
    date: String, // TODO - another format?
    /// Path to image (if existent)
    image: Option<String>,
    /// Links to project
    links: HashMap<String, String>,

    /// Raw cached project as json
    cached_json: serde_json::Value,
}

impl ProjectData {
    fn metadata(&self) -> serde_json::Value {
        json!({
            "name": self.name,
            "description": self.description,
            "date": self.date,
            "image": self.image,
            "links": self.links,
        })
    }
}

impl Default for ProjectData {
    fn default() -> Self {
        ProjectData {
            name: "".to_string(),
            description: "".to_string(),
            date: "".to_string(),
            image: None,
            links: HashMap::new(),
            cached_json: json!({}),
        }
    }
}

pub struct ProjectPage {
    index: String,
    projects: Vec<ProjectData>,
    raw_projects: serde_json::Value,
}

impl ProjectPage {
    pub fn new() -> Self {
        let raw_projects = util::read_yaml_to_json(
            &util::read_embedded_text::<EmbeddedProjectsFiles>("projects.yaml").unwrap(),
        )
        .unwrap();
        let raw_projects_list = &raw_projects["projects"].as_array().unwrap();
        let mut project_list: Vec<ProjectData> = Vec::new();
        for project_data in raw_projects_list.iter().rev() {
            let project: ProjectData = serde_json::value::from_value(project_data.clone()).unwrap();
            project_list.push(project);
        }
        ProjectPage {
            index: util::read_embedded_text::<EmbeddedProjectsFiles>("projects.html").unwrap(),
            projects: project_list,
            raw_projects: raw_projects,
        }
    }

    pub fn project_metadata(&self) -> serde_json::Value {
        return json!({
            "projects": self.projects.iter().map(|proj| proj.metadata()).collect::<Vec<serde_json::Value>>()
        });
    }
}

pub async fn visit_projects<'a>(State(site): State<SharedSite<'a>>) -> Html<String> {
    match site.page_cache.retrieve("projects") {
        Ok(page) => page,
        Err(_) => {
            let proj_metadata = site.pages.projects.project_metadata();
            site.page_cache.update(
                "projects",
                Html(site.render_page(&site.pages.projects.index, &proj_metadata)),
            )
        }
    }
}
