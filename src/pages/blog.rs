use axum::{extract::Path, extract::State, response::Html};
use log::*;
use rust_embed::RustEmbed;
use serde_json::json;

use crate::site::SharedSite;
use crate::util::*;

#[derive(RustEmbed)]
#[folder = "content/pages/blog"]
pub struct BlogFiles;

pub struct BlogData {
    /// Title of the blog
    name: String,
    /// Description of blog
    blurb: String,
    /// Actual local path to blog
    path: String,
    /// Read markdown of blog entry
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
        let index = read_embedded_text::<BlogFiles>("blog.html").unwrap();
        let blog_page = read_embedded_text::<BlogFiles>("blog_article.html").unwrap();

        // Parse blogs
        //// Read yaml metadata
        let article_metadata: serde_json::Value =
            read_yaml_to_json(&read_embedded_text::<BlogFiles>("article_metadata.yaml").unwrap())
                .unwrap();

        //// Convert to array
        let mut blogs: Vec<BlogData> = Vec::new();
        let articles = &article_metadata["articles"].as_array().unwrap();
        for article_data in articles.iter().rev() {
            let article_data = article_data.as_object().unwrap();
            let name = article_data["name"].as_str().unwrap_or("").to_string();
            let blurb = article_data["blurb"].as_str().unwrap_or("").to_string();
            let path = article_data["path"].as_str().unwrap_or("").to_string();
            let markdown =
                read_embedded_text::<BlogFiles>(&format!("articles/{path}/{path}.md")).unwrap();

            blogs.push(BlogData {
                name: name,
                blurb: blurb,
                path: path,
                markdown: markdown,
            });
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

/// Visit blog page
pub async fn visit_blog_index<'a>(State(site): State<SharedSite<'a>>) -> Html<String> {
    let blog_metadata = site.blog_indexer.blog_metadata();
    Html(site.render_page(&site.blog_indexer.index, &blog_metadata))
}

/// Visit individual blog
pub async fn visit_blog<'a>(
    Path(blog): Path<String>,
    State(site): State<SharedSite<'a>>,
) -> Html<String> {
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

/// Get local blog resource
pub async fn get_blog_resource<'a>(
    Path((blog, resource)): Path<(String, String)>,
    State(_site): State<SharedSite<'a>>,
) -> Vec<u8> {
    let blog_resource: String = format!("articles/{blog}/{resource}");
    match read_embedded_data::<BlogFiles>(&blog_resource) {
        Ok(data) => data,
        Err(_) => {
            error!("Unable to render blog resource {blog_resource}");
            vec![]
        }
    }
}
