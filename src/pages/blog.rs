use axum::{extract::Path, extract::State, response::Html};
use log::*;
use rss::{ChannelBuilder, Item, ItemBuilder};
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::pages::error::visit_404;
use crate::site::SharedSite;
use crate::util::*;

#[derive(RustEmbed)]
#[folder = "content/pages/blog"]
struct EmbeddedBlogFiles;

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct BlogData {
    /// Title of the blog
    name: String,
    /// Description of blog
    blurb: String,
    /// Date
    date: String,
    /// Actual local path to blog
    path: String,

    /// Read markdown of blog entry
    markdown: String,
    /// Raw cached blog json
    cached_json: serde_json::Value,
}

impl Default for BlogData {
    fn default() -> Self {
        BlogData {
            name: "".to_string(),
            blurb: "".to_string(),
            date: "".to_string(),
            path: "".to_string(),
            markdown: "".to_string(),
            cached_json: json!({}),
        }
    }
}

pub struct BlogIndexer {
    index: String,
    article: String,
    blogs: Vec<BlogData>,
    cached_rss: String,
}

impl BlogIndexer {
    pub fn new() -> Self {
        // Parse pages
        let index = read_embedded_text::<EmbeddedBlogFiles>("blogs.html").unwrap();
        let article = read_embedded_text::<EmbeddedBlogFiles>("article.html").unwrap();

        // Parse blogs
        //// Read yaml metadata
        let article_metadata: serde_json::Value = read_yaml_to_json(
            &read_embedded_text::<EmbeddedBlogFiles>("article_metadata.yaml").unwrap(),
        )
        .unwrap();
        //// Read articles list
        let mut blogs: Vec<BlogData> = Vec::new();
        let articles = &article_metadata["articles"].as_array().unwrap();
        for article_data in articles.iter().rev() {
            let mut blog: BlogData = serde_json::value::from_value(article_data.clone()).unwrap();
            let path = blog.path.clone();
            blog.markdown =
                read_embedded_text::<EmbeddedBlogFiles>(&format!("articles/{path}/{path}.md"))
                    .unwrap();
            blog.cached_json = article_data.clone();
            blogs.push(blog);
        }
        //// Parse into rss feed
        let mut channel = ChannelBuilder::default();
        channel
            .title("hachha.dev".to_string())
            .link("https://hachha.dev".to_string())
            .description("hachha.dev blog feed".to_string());
        let mut items = Vec::new();
        for blog in blogs.iter() {
            let item: Item = ItemBuilder::default()
                .title(Some(blog.name.clone()))
                .description(Some(blog.blurb.clone()))
                .link(Some(format!(
                    "https://hachha.dev/blog/{}",
                    blog.path.as_str()
                )))
                .pub_date(Some(
                    (match chrono::DateTime::parse_from_str(
                        &format!("{} 00:00:00+0000", blog.date.as_str()),
                        "%F %H:%M:%S%z",
                    ) {
                        Ok(pub_date) => pub_date,
                        Err(e) => {
                            error!("Blog date error: {} -> {}", blog.date, e);
                            chrono::offset::Utc::now().into()
                        }
                    })
                    .to_rfc2822(),
                ))
                .build();
            items.push(item);
        }
        channel.items(items);
        let cached_rss = channel.build().to_string();

        BlogIndexer {
            index,
            article,
            blogs,
            cached_rss,
        }
    }

    fn blog_metadata(&self) -> serde_json::Value {
        let mut metadata = json!({});
        let mut blogs: Vec<serde_json::Value> = Vec::new();
        for blog in self.blogs.iter() {
            blogs.push(blog.cached_json.clone());
        }
        metadata["blogs"] = serde_json::Value::Array(blogs);
        metadata
    }

    fn get_blog(&self, path: &str) -> Option<&BlogData> {
        for other_blog in self.blogs.iter() {
            if path == other_blog.path {
                return Some(other_blog);
            }
        }
        None
    }
}

/// Visit blog page
pub async fn visit_blog_index<'a>(State(site): State<SharedSite<'a>>) -> Html<String> {
    match site.page_cache.retrieve("blog") {
        Ok(page) => page,
        Err(_) => {
            let blog_metadata = site.pages.blog_indexer.blog_metadata();
            site.page_cache.update(
                "blog",
                Html(site.render_page(&site.pages.blog_indexer.index, &blog_metadata)),
            )
        }
    }
}

/// Visit individual blog
pub async fn visit_blog<'a>(
    Path(blog): Path<String>,
    State(site): State<SharedSite<'a>>,
) -> Html<String> {
    // Visit index
    if blog.is_empty() {
        return visit_blog_index(State(site)).await;
    }
    let full_blog_path: String = format!("blog/{blog}");
    match site.page_cache.retrieve(&full_blog_path) {
        Ok(page) => {
            return page;
        }
        Err(_) => {
            if let Some(blog) = site.pages.blog_indexer.get_blog(&blog) {
                let mut blog_metadata = blog.cached_json.clone();
                blog_metadata["blog-content"] = serde_json::Value::String(blog.markdown.clone());
                return site.page_cache.update(
                    &full_blog_path,
                    Html(site.render_page(&site.pages.blog_indexer.article, &blog_metadata)),
                );
            };
        }
    };

    error!("Visiting invalid blog {}", full_blog_path);
    return visit_404(State(site)).await;
}

/// Get local blog resource
pub async fn get_blog_resource<'a>(
    Path((blog, resource)): Path<(String, String)>,
    State(_site): State<SharedSite<'a>>,
) -> Vec<u8> {
    let blog_resource: String = format!("articles/{blog}/{resource}");
    match read_embedded_data::<EmbeddedBlogFiles>(&blog_resource) {
        Ok(data) => data,
        Err(_) => {
            error!("Unable to render blog resource {blog_resource}");
            vec![]
        }
    }
}

/// Get blog as rss
pub async fn visit_blog_rss<'a>(State(site): State<SharedSite<'a>>) -> String {
    site.pages.blog_indexer.cached_rss.clone()
}
