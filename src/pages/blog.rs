//! Blog pages.

use super::*;

use atom_syndication as atom;

/// The blogs page and subpages.
pub struct BlogsPages {
    index: String,
    article: String,
    blogs: Blogs,
    feed: String,
    metadata: serde_json::Value,
}

impl BlogsPages {
    /// Generate new blogs pages.
    pub fn new(packed_data: Arc<PackedData>) -> Result<Self> {
        // Parse pages.
        let index = util::read_embedded_text::<EmbeddedPages>("blog/blogs.html")?;
        let article = util::read_embedded_text::<EmbeddedPages>("blog/articles/article.html")?;
        let mut blogs = Blogs::default();
        for (path, _data) in packed_data.iter() {
            if path.ends_with("blog.toml") {
                let path = String::from(path);
                let mut blog = packed_data.read_toml::<Blog>(&path)?;
                let dir = String::from(match std::path::Path::new(&path).parent() {
                    Some(parent) => {
                        blog.directory = match parent.file_name() {
                            Some(fname) => fname.to_string_lossy().into_owned(),
                            None => {
                                tracing::warn!(
                                    "Blog has invalid slug: {}, defaulting to uri.",
                                    blog.uri
                                );
                                blog.uri.clone()
                            }
                        };
                        parent.to_string_lossy()
                    }
                    None => {
                        tracing::error!("Unable to find directory for blog at {path}.");
                        continue;
                    }
                });
                let article_path = format!("{dir}/blog.md");
                blog.markdown = match packed_data.read_text(&article_path) {
                    Ok(md) => md,
                    Err(e) => {
                        tracing::error!("Failed to read parsed blog: {}", e);
                        continue;
                    }
                };
                blog.metadata = util::to_json(&blog)?;
                blogs.push(blog);
            }
        }
        blogs.sort();
        blogs.reverse();

        // Parse into atom feed.
        let mut feed_builder = atom::FeedBuilder::default();
        feed_builder
            .title("hachha.dev")
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
            let timestamp: chrono::DateTime<chrono::FixedOffset> = blog
                .date
                .and_time(chrono::NaiveTime::default())
                .and_local_timezone(chrono::Utc)
                .latest()
                .ok_or(anyhow!("Unable to convert dt {} to utc.", blog.date))?
                .into();
            let entry = atom::EntryBuilder::default()
                .title(blog.name.clone())
                .summary(Some(blog.blurb.clone().into()))
                .link(
                    atom::LinkBuilder::default()
                        .href(format!("https://hachha.dev/blog/{}", blog.uri.as_str()))
                        .title(blog.name.clone())
                        .build(),
                )
                .published(timestamp)
                .updated(timestamp)
                .categories(
                    blog.tags
                        .iter()
                        .map(|tag| atom::CategoryBuilder::default().term(tag).build())
                        .collect::<Vec<atom::Category>>(),
                )
                .build();
            entries.push(entry);
        }
        feed_builder.entries(entries);
        let feed = feed_builder.build().to_string();

        // Generate blog metadata. Darken every other entry.
        let mut metadata = json!({});
        let mut blog_metadata: Vec<serde_json::Value> = Vec::new();
        let mut all_tags: BTreeSet<String> = BTreeSet::new();
        for (i, blog) in blogs.iter().enumerate() {
            let mut meta = blog.metadata.clone();
            util::merge_json(&mut meta, &json!({"darken": i % 2 == 0, "path": blog.uri}))?;
            blog_metadata.push(meta);
            for other_tag in &blog.tags {
                if !all_tags.contains(other_tag) {
                    all_tags.insert(other_tag.into());
                }
            }
        }
        metadata["blogs"] = serde_json::Value::Array(blog_metadata);
        metadata["tags"] = serde_json::Value::Array(
            all_tags
                .into_iter()
                .map(|tag| serde_json::Value::String(tag))
                .collect(),
        );
        metadata["tag"] = serde_json::Value::String("".into());

        Ok(BlogsPages {
            index,
            article,
            blogs,
            feed,
            metadata,
        })
    }

    fn get_blog(&self, path: &str) -> Option<&Blog> {
        for other_blog in self.blogs.iter() {
            if path == other_blog.uri {
                return Some(other_blog);
            }
        }
        None
    }
}

/// Parsed blog list.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Blogs {
    /// Blog articles.
    articles: Vec<Blog>,
}

impl Default for Blogs {
    fn default() -> Self {
        Self {
            articles: Vec::new(),
        }
    }
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

/// Parsed blog data.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Blog {
    /// Title of the blog.
    name: String,
    /// Description of blog.
    blurb: String,
    /// Date blog was written.
    date: chrono::NaiveDate,
    /// Relative URI for blog.
    #[serde(alias = "article")]
    uri: String,
    /// Resource path directory name (e.g., 2026-02-01-cool_blog).
    #[serde(default)]
    directory: String,
    /// Tags.
    #[serde(default)]
    tags: BTreeSet<String>,
    /// Read markdown of blog entry.
    #[serde(skip)]
    markdown: String,
    /// Raw cached blog json.
    #[serde(skip)]
    metadata: serde_json::Value,
}

impl Default for Blog {
    fn default() -> Self {
        Blog {
            name: "".to_string(),
            blurb: "".to_string(),
            date: chrono::NaiveDate::default(),
            uri: "".to_string(),
            directory: "".into(),
            tags: BTreeSet::new(),
            markdown: "".to_string(),
            metadata: json!({}),
        }
    }
}

impl std::cmp::PartialOrd for Blog {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.date.partial_cmp(&other.date)
    }
}

impl std::cmp::Ord for Blog {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.date.cmp(&other.date)
    }
}

/// Endpoint for blogs index page.
pub async fn visit_blog_index(State(site): State<Site>) -> RenderedHtml {
    site.clone()
        .page_cache()
        .retrieve_or_update("blog", async move {
            site.render_page(&site.pages().blogs.index, &site.pages().blogs.metadata)
        })
        .await
}

/// Endpoint for individual blogs.
pub async fn visit_blog(
    Path(blog): Path<String>,
    State(site): State<Site>,
    headers: HeaderMap,
) -> RenderedHtml {
    // Visit index
    if blog.is_empty() {
        return visit_blog_index(State(site)).await;
    }

    // Check for blog.
    let full_blog_path: String = format!("blog/{blog}");
    match site.pages().blogs.get_blog(&blog) {
        Some(blog) => {
            site.clone()
                .page_cache()
                .retrieve_or_update(&full_blog_path, async move {
                    let mut blog_metadata = blog.metadata.clone();
                    blog_metadata["blog-content"] =
                        serde_json::Value::String(blog.markdown.clone());
                    site.render_page(&site.pages().blogs.article, &blog_metadata)
                })
                .await
        }
        None => {
            error::visit_404_internal(format!("/blog/{blog}"), State(site), Some(headers)).await
        }
    }
}

/// Visit tag.
pub async fn visit_tag(Path(tag): Path<String>, State(site): State<Site>) -> RenderedHtml {
    visit_tag_internal(Some(tag.as_str()), State(site)).await
}

/// Visit tag internal.
pub async fn visit_tag_internal(tag: Option<&str>, State(site): State<Site>) -> RenderedHtml {
    let page = format!("tags/{tag:?}");

    // Collect metadata.
    let mut metadata = site.pages().blogs.metadata.clone();
    let mut blog_metadata: Vec<serde_json::Value> = Vec::new();
    for (_, blog) in site.pages().blogs.blogs.iter().enumerate() {
        for other_tag in &blog.tags {
            if Some(other_tag.as_str()) == tag {
                let mut meta = blog.metadata.clone();
                util::merge_json(
                    &mut meta,
                    &json!({"darken": blog_metadata.len() % 2 == 0, "path": blog.uri}),
                )
                .ok();
                blog_metadata.push(meta);
            }
        }
    }
    metadata["tag"] = match &tag {
        Some(tag) => serde_json::Value::String((*tag).to_owned()),
        None => serde_json::Value::String("".into()),
    };
    metadata["blogs"] = serde_json::Value::Array(blog_metadata);

    site.clone()
        .page_cache()
        .retrieve_or_update(&page, async move {
            site.render_page(&site.pages().blogs.index, &metadata)
        })
        .await
}

/// Get local blog resource.
pub async fn get_blog_resource(
    Path((blog, resource)): Path<(String, String)>,
    State(site): State<Site>,
) -> impl axum::response::IntoResponse {
    // Blog resource referenced by article uri, not file path, so we do a lookup.
    let mut blog_path = blog.clone();
    for other_blog in &*site.pages().blogs.blogs {
        if other_blog.uri == blog {
            blog_path = other_blog.directory.clone();
            break;
        }
    }
    let blog_resource: String = format!("pages/blog/articles/{blog_path}/{resource}");

    let data = match site.packed_data().read_data(&blog_resource) {
        Ok(data) => data,
        Err(_) => {
            tracing::error!("Unable to render blog resource {blog_resource}");
            EmbeddedData::empty()
        }
    };
    return crate::util::adjust_content_header(resource, data);
}

/// Get blog as atom feed.
pub async fn visit_blog_feed(State(site): State<Site>) -> impl axum::response::IntoResponse {
    (
        [(axum::http::header::CONTENT_TYPE, "application/atom+xml")],
        site.pages().blogs.feed.clone(),
    )
}
