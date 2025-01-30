//! Blog pages.

use super::*;

use atom_syndication as atom;

/// Parsed blog list.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Blogs {
    articles: Vec<Blog>,
}

impl std::ops::Deref for Blogs {
    type Target = Vec<Blog>;
    fn deref(&self) -> &Self::Target {
        &self.articles
    }
}

impl std::ops::DerefMut for Blogs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.articles
    }
}

/// Embedded blog files.
#[derive(RustEmbed)]
#[folder = "content/pages/blog"]
struct EmbeddedBlogFiles;

/// Parsed blog data.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Blog {
    /// Title of the blog.
    name: String,
    /// Description of blog.
    blurb: String,
    /// Date blog was written.
    date: String,
    /// Actual local path to blog.
    path: String,

    /// Read markdown of blog entry
    #[serde(skip)]
    markdown: String,
    /// Raw cached blog json
    #[serde(skip)]
    cached_json: serde_json::Value,
}

impl Default for Blog {
    fn default() -> Self {
        Blog {
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
    blogs: Blogs,
    cached_feed: String,
}

impl BlogIndexer {
    /// Get new blog indexer.
    pub fn new() -> Self {
        // Parse pages.
        let index = util::read_embedded_text::<EmbeddedBlogFiles>("blogs.html")
            .expect("Must have blogs.html page!");
        let article = util::read_embedded_text::<EmbeddedBlogFiles>("article.html")
            .expect("Must have article.html page!");

        // Parse blogs.
        let mut blogs =
            util::read_embedded_toml::<Blogs, EmbeddedBlogFiles>("articles/articles.toml")
                .expect("Unable to parse blogs!");
        blogs.reverse();

        // Read articles list.
        for blog in blogs.iter_mut() {
            let raw_path = &blog.path;
            let path = format!("articles/{raw_path}/{raw_path}.md");
            blog.markdown = util::read_embedded_text::<EmbeddedBlogFiles>(&path)
                .expect(&format!("Blog path doesn't exist: {path}"));
            blog.cached_json =
                util::to_json(blog).expect(&format!("Unable to convert blog to json {blog:?}"));
        }

        // Parse into atom feed.
        let mut feed = atom::FeedBuilder::default();
        feed.title("hachha.dev")
            .author(atom::PersonBuilder::default().name("Harrison Hall").build())
            .link(
                atom::LinkBuilder::default()
                    .href("https://hachha.dev")
                    .title("hachha.dev".to_string())
                    .build(),
            )
            .icon("https://hachha.dev/media/catman.png".to_string())
            .subtitle(Some("hachha.dev blog feed".to_string().into()));
        let mut entries = Vec::new();
        for blog in blogs.iter() {
            let timestamp = match chrono::DateTime::parse_from_str(
                &format!("{} 00:00:00+0000", blog.date.as_str()),
                "%F %H:%M:%S%z",
            ) {
                Ok(pub_date) => pub_date,
                Err(e) => {
                    error!("Blog date error: {} -> {}", blog.date, e);
                    chrono::offset::Utc::now().into()
                }
            };
            let entry: atom::Entry = atom::EntryBuilder::default()
                .title(blog.name.clone())
                .summary(Some(blog.blurb.clone().into()))
                .link(
                    atom::LinkBuilder::default()
                        .href(format!("https://hachha.dev/blog/{}", blog.path.as_str()))
                        .title(blog.name.clone())
                        .build(),
                )
                .published(timestamp)
                .updated(timestamp)
                .build();
            entries.push(entry);
        }
        feed.entries(entries);
        let cached_feed = feed.build().to_string();

        BlogIndexer {
            index,
            article,
            blogs,
            cached_feed,
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

    fn get_blog(&self, path: &str) -> Option<&Blog> {
        for other_blog in self.blogs.iter() {
            if path == other_blog.path {
                return Some(other_blog);
            }
        }
        None
    }
}

/// Visit blog page
pub async fn visit_blog_index(State(site): State<SharedSite>) -> Html<String> {
    match site.page_cache().retrieve("blog").await {
        Ok(page) => page,
        Err(_) => {
            let blog_metadata = site.pages().blog_indexer.blog_metadata();
            site.page_cache()
                .update(
                    "blog",
                    Html(site.render_page(&site.pages().blog_indexer.index, &blog_metadata)),
                )
                .await
        }
    }
}

/// Visit individual blog
pub async fn visit_blog(Path(blog): Path<String>, State(site): State<SharedSite>) -> Html<String> {
    // Visit index
    if blog.is_empty() {
        return visit_blog_index(State(site)).await;
    }
    let full_blog_path: String = format!("blog/{blog}");
    match site.page_cache().retrieve(&full_blog_path).await {
        Ok(page) => {
            return page;
        }
        Err(_) => {
            if let Some(blog) = site.pages().blog_indexer.get_blog(&blog) {
                let mut blog_metadata = blog.cached_json.clone();
                blog_metadata["blog-content"] = serde_json::Value::String(blog.markdown.clone());
                return site
                    .page_cache()
                    .update(
                        &full_blog_path,
                        Html(site.render_page(&site.pages().blog_indexer.article, &blog_metadata)),
                    )
                    .await;
            };
        }
    };

    error!("Visiting invalid blog {}", full_blog_path);
    return error::visit_404(State(site)).await;
}

/// Get local blog resource.
pub async fn get_blog_resource(
    Path((blog, resource)): Path<(String, String)>,
    State(_site): State<SharedSite>,
) -> Vec<u8> {
    let blog_resource: String = format!("articles/{blog}/{resource}");
    match util::read_embedded_data::<EmbeddedBlogFiles>(&blog_resource) {
        Ok(data) => data,
        Err(_) => {
            error!("Unable to render blog resource {blog_resource}");
            vec![]
        }
    }
}

/// Get blog as atom feed.
pub async fn visit_blog_feed(State(site): State<SharedSite>) -> impl axum::response::IntoResponse {
    (
        [(axum::http::header::CONTENT_TYPE, "application/atom+xml")],
        site.pages().blog_indexer.cached_feed.clone(),
    )
}
