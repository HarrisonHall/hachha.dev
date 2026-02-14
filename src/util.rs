//! Utils.

use super::*;

use rust_embed::RustEmbed;

/// Embedded data type.
#[derive(Clone)]
pub struct EmbeddedData(Cow<'static, [u8]>);

impl EmbeddedData {
    pub fn empty() -> Self {
        Self(Cow::from(&[]))
    }
}

impl std::ops::Deref for EmbeddedData {
    type Target = Cow<'static, [u8]>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl axum::response::IntoResponse for EmbeddedData {
    fn into_response(self) -> axum::response::Response {
        self.0.into_response()
    }
}

/// Packed data.
#[derive(Clone)]
pub struct PackedData {
    data: Arc<HashMap<String, EmbeddedData>>,
}

impl PackedData {
    pub fn new(path: impl AsRef<std::path::Path>) -> Result<Self> {
        let tar_gz = match std::fs::File::open(path.as_ref()) {
            Ok(tgz) => tgz,
            Err(e) => bail!("Failed to open packed data: {e}"),
        };
        let tar = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = tar::Archive::new(tar);

        let entries = match archive.entries() {
            Ok(entries) => entries,
            Err(e) => {
                bail!("Failed to generate entries of packed data: {e}");
            }
        };
        let mut data = HashMap::new();
        for entry in entries {
            if let Ok(mut entry) = entry {
                if let Ok(entry_path) = entry.path() {
                    tracing::trace!("PackedData found file: {}", entry_path.to_string_lossy());
                    let path = entry_path.to_string_lossy().into_owned();
                    let mut full_file = match entry.header().size() {
                        Ok(size) => Vec::with_capacity(size as usize),
                        Err(_) => {
                            tracing::warn!("Failed to view file size for packed file `{path}`.");
                            Vec::new()
                        }
                    };
                    match entry.read_to_end(&mut full_file) {
                        Ok(_bytes) => {
                            data.insert(path, EmbeddedData(Cow::Owned(full_file.into())));
                        }
                        Err(e) => {
                            tracing::error!("Failed to read file `{path}`: {e}")
                        }
                    }
                }
            }
        }

        Ok(Self {
            data: Arc::new(data),
        })
    }

    pub fn iter<'a>(&'a self) -> std::collections::hash_map::Iter<'a, String, EmbeddedData> {
        self.data.iter()
    }

    /// Read data from packed file.
    pub fn read_data(&self, path: impl AsRef<str>) -> Result<EmbeddedData> {
        let path = path
            .as_ref()
            .trim_start_matches("./")
            .trim_start_matches("/")
            .trim_start_matches("content/");

        // In debug, read from file each time.
        if cfg!(debug_assertions) {
            let path = format!("./content/{path}");
            return match std::fs::read(&path) {
                Ok(data) => Ok(EmbeddedData(Cow::Owned(data.into()))),
                Err(e) => {
                    tracing::warn!("Failed to read file `{path}` in debug: {e}");
                    bail!("FileNotFound")
                }
            };
        }

        if let Some(data) = self.data.get(path) {
            return Ok(data.clone());
        }

        tracing::warn!("Did not find embedded data: `{}`.", path);
        bail!("FileNotFound")
    }

    /// Parse embedded file to text.
    pub fn read_text(&self, path: impl AsRef<str>) -> Result<String> {
        let data = self.read_data(path.as_ref())?;
        match std::str::from_utf8(&data) {
            Ok(file) => Ok(file.to_string()),
            _ => bail!("Unable to convert binary file {} to string", path.as_ref()),
        }
    }

    /// Read toml.
    pub fn read_toml<T: DeserializeOwned>(&self, path: impl AsRef<str>) -> Result<T> {
        let data = self.read_data(path.as_ref())?;
        match std::str::from_utf8(&data) {
            Ok(file) => toml::from_str::<T>(file)
                .map_err(|e| anyhow!("read_embedded_toml error for {}: {e}", path.as_ref())),
            _ => bail!("Unable to convert binary file {} to string", path.as_ref()),
        }
    }
}

/// Rendered html type.
#[derive(Clone)]
pub struct RenderedHtml(Arc<Html<String>>);

impl RenderedHtml {
    pub fn new(html: impl Into<String>) -> Self {
        Self(Arc::new(Html(html.into())))
    }
}

impl axum::response::IntoResponse for RenderedHtml {
    fn into_response(self) -> axum::response::Response {
        (*self.0).clone().into_response()
    }
}

/// Initialize logging.
pub fn init_logging(debug: bool) -> Result<()> {
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::fmt;
    use tracing_subscriber::prelude::*;

    let level = if cfg!(debug_assertions) {
        LevelFilter::TRACE
    } else {
        match debug {
            true => LevelFilter::DEBUG,
            false => LevelFilter::INFO,
        }
    };
    let format_layer = fmt::layer()
        .with_level(true)
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .compact();
    let filter_layer = tracing_subscriber::filter::Targets::new()
        .with_default(LevelFilter::TRACE)
        .with_target("handlebars", LevelFilter::WARN)
        .with_target("globset", LevelFilter::WARN)
        .with_target("axum", LevelFilter::WARN)
        .with_target("tower_http", LevelFilter::WARN)
        .with_target("mio", LevelFilter::WARN)
        .with_target("hachha_dev", level);
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(format_layer)
        .init();

    Ok(())
}

/// Read embedded file as data.
#[allow(unused)]
pub fn read_embedded_data<Embed: RustEmbed>(path: impl AsRef<str>) -> Result<EmbeddedData> {
    match Embed::get(path.as_ref()) {
        Some(file) => Ok(EmbeddedData(file.data)),
        None => bail!("Unable to find file {}", path.as_ref()),
    }
}

/// Parse embedded file to text.
#[allow(unused)]
pub fn read_embedded_text<Embed: RustEmbed>(path: impl AsRef<str>) -> Result<String> {
    let data = read_embedded_data::<Embed>(path.as_ref())?;
    match std::str::from_utf8(&data) {
        Ok(file) => Ok(file.to_string()),
        _ => bail!("Unable to convert binary file {} to string", path.as_ref()),
    }
}

/// Read embedded toml.
#[allow(unused)]
pub fn read_embedded_toml<T: DeserializeOwned, Embed: RustEmbed>(
    path: impl AsRef<str>,
) -> Result<T> {
    let data = read_embedded_data::<Embed>(path.as_ref())?;
    match std::str::from_utf8(&data) {
        Ok(file) => toml::from_str::<T>(file)
            .map_err(|e| anyhow!("read_embedded_toml error for {}: {e}", path.as_ref())),
        _ => bail!("Unable to convert binary file {} to string", path.as_ref()),
    }
}

/// Serialize to json.
pub fn to_json<T: Serialize>(value: &T) -> Result<serde_json::Value> {
    Ok(serde_json::to_value(value)?)
}

/// Merge json values.
pub fn merge_json(a: &mut serde_json::Value, b: &serde_json::Value) -> Result<()> {
    match (a, &b) {
        (a @ &mut serde_json::Value::Object(_), &serde_json::Value::Object(b)) => {
            let a = match a.as_object_mut() {
                Some(a) => a,
                None => bail!("Unable to merge json."),
            };
            for (k, v) in b {
                merge_json(a.entry(k).or_insert(serde_json::Value::Null), &v)?;
            }
        }
        (a, b) => *a = (*b).clone(),
    }
    Ok(())
}

/// Add correct header for file.
pub fn adjust_content_header(
    resource: impl AsRef<str>,
    response: impl axum::response::IntoResponse,
) -> impl axum::response::IntoResponse {
    let resource = resource.as_ref();
    let content_type = if resource.ends_with(".css") {
        "text/css"
    } else if resource.ends_with(".epub") {
        "application/epub+zip"
    } else if resource.ends_with(".gif") {
        "image/gif"
    } else if resource.ends_with(".html") {
        "text/html"
    } else if resource.ends_with(".ico") {
        "image/vdn.microsoft.icon"
    } else if resource.ends_with(".jpg") {
        "image/jpeg"
    } else if resource.ends_with(".jpeg") {
        "image/jpeg"
    } else if resource.ends_with(".js") {
        "text/javascript"
    } else if resource.ends_with(".md") {
        "text/plain"
    } else if resource.ends_with(".otf") {
        "font/otf"
    } else if resource.ends_with(".png") {
        "image/png"
    } else if resource.ends_with(".toml") {
        "text/plain"
    } else if resource.ends_with(".ttf") {
        "font/ttf"
    } else if resource.ends_with(".txt") {
        "text/plain"
    } else if resource.ends_with(".woff") {
        "font/woff"
    } else if resource.ends_with(".woff2") {
        "font/woff2"
    } else if resource.ends_with(".xhtml") {
        "application/xhtml+xml"
    } else {
        "application/octet-stream"
    };
    return ([(axum::http::header::CONTENT_TYPE, content_type)], response);
}
