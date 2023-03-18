use std::path::PathBuf;
use std::sync::Arc;
use std::borrow::Cow;

use clap::Parser;
use handlebars::Handlebars;
use log::*;
use rust_embed::RustEmbed;

use axum::{
    extract::Path,
    extract::State,
    routing::{get, post, get_service},
    response::Html,
    Json, Router
};
use serde_json::json;
use serde_yaml;


use crate::site::Site;
use crate::util::*;


#[derive(RustEmbed)]
#[folder = "content/pages/blog"]
pub struct BlogFiles;


pub struct BlogData {
    name: String,
    blurb: String,
    path: String,
    markdown: String,
}

impl BlogData {
    pub fn metadata(&self) -> serde_json::Value {
        json!({
            "name": self.name,
            "blurb": self.blurb,
            "path": self.path,
            "markdown": self.markdown,
            "blog-content": self.markdown,
        })
    }
}

pub struct BlogIndexer {
    index: String,
    blog_page: String,
    blogs: Vec<BlogData>,
}

impl BlogIndexer {
    pub fn new() -> Self {
        // Parse pages
        let index = read_embedded_text::<BlogFiles>("index.html").unwrap();
        let blog_page = read_embedded_text::<BlogFiles>("blog_page.html").unwrap();

        // Parse blogs
        //// Read yaml metadata
        let article_metadata: serde_json::Value = read_yaml_to_json(
            &read_embedded_text::<BlogFiles>("article_metadata.yaml").unwrap()
        ).unwrap();

        //// Convert to array
        let mut blogs: Vec<BlogData> = Vec::new();
        let articles = &article_metadata["articles"].as_array().unwrap();
        for article_data in articles.iter() {
            let article_data = article_data.as_object().unwrap();
            let name = article_data["name"].as_str().unwrap_or("").to_string();
            let blurb = article_data["blurb"].as_str().unwrap_or("").to_string();
            let path = article_data["path"].as_str().unwrap_or("").to_string();
            let markdown = read_embedded_text::<BlogFiles>(&format!("articles/{path}/{path}.md")).unwrap();
            
            blogs.push(
                BlogData {
                    name: name,
                    blurb: blurb,
                    path: path,
                    markdown: markdown,
                }
            );
        }

        BlogIndexer {
            index: index,
            blog_page: blog_page,
            blogs: blogs,
        }
    }

    pub fn blog_metadata(&self) -> serde_json::Value {
        let mut metadata = json!({});
        let mut blogs: Vec<serde_json::Value> = Vec::new();
        for blog in self.blogs.iter() {
            blogs.push(blog.metadata());
        }
        metadata["blogs"] = serde_json::Value::Array(blogs);
        metadata
    }
}


pub async fn visit_blog_index<'a>(State(site): State<Arc<Site<'a>>>) -> Html<String> {
        let blog_metadata = site.blog_indexer.blog_metadata();
        Html(site.render_page(&site.blog_indexer.index, &blog_metadata))
}

pub async fn visit_blog<'a>(Path(blog): Path<String>, State(site): State<Arc<Site<'a>>>) -> Html<String> {
    for other_blog in site.blog_indexer.blogs.iter() {
        if blog == other_blog.path {
            let blog_metadata = other_blog.metadata();
            let rendered_page = site.render_page(&site.blog_indexer.blog_page, &blog_metadata);
            return Html(rendered_page);
        }
    }

    error!("Visiting invalid blog {}", blog);
    Html("".to_owned())
}

pub async fn get_blog_resource<'a>(Path(blog): Path<String>, Path(resource): Path<String>, State(site): State<Arc<Site<'a>>>) -> Vec<u8> {
    // TODO get resource from embedded data
    return Vec::new();
}