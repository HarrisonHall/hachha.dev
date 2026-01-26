//! Utils.

use colored::{ColoredString, Colorize};
use rust_embed::RustEmbed;

use super::*;

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

/// Logging struct.
static LOGGER_SETUP: Once = Once::new();

/// Sim logger.
pub struct Logger {}

impl Logger {
    /// Create and set a new logger.
    pub fn init(level: impl Into<String>) -> Result<()> {
        LOGGER_SETUP.call_once(|| {
            let logger = Box::new(Self {});
            log::set_logger(Box::leak(logger)).ok();
        });
        let level = level.into().to_uppercase();
        match level.as_str() {
            "TRACE" => log::set_max_level(log::LevelFilter::Trace),
            "DEBUG" => log::set_max_level(log::LevelFilter::Debug),
            "INFO" => log::set_max_level(log::LevelFilter::Info),
            "Warn" => log::set_max_level(log::LevelFilter::Warn),
            "Error" => log::set_max_level(log::LevelFilter::Error),
            _ => {
                log::warn!("Invalid log level: {level}");
                log::set_max_level(log::LevelFilter::Error);
            }
        }
        Ok(())
    }

    // pub fn layer() -> LoggerLayer {
    //     LoggerLayer::new()
    // }

    /// Get string for a level.
    fn get_level_string(&self, level: log::Level) -> &'static str {
        match level {
            log::Level::Trace => "TRC",
            log::Level::Debug => "DBG",
            log::Level::Info => "INF",
            log::Level::Warn => "WRN",
            log::Level::Error => "ERR",
        }
    }

    /// Get string for a level, ANSI colored.
    fn get_level_string_colored(&self, level: log::Level) -> ColoredString {
        let level = match level {
            log::Level::Trace => self.get_level_string(level).cyan(),
            log::Level::Debug => self.get_level_string(level).magenta(),
            log::Level::Info => self.get_level_string(level).blue(),
            log::Level::Warn => self.get_level_string(level).yellow(),
            log::Level::Error => self.get_level_string(level).red(),
        };
        level.bold()
    }
}

impl log::Log for Logger {
    /// Check if log message w/ metadata can be logged.
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        // Return early if level is low
        if metadata.level() > log::max_level() {
            return false;
        }
        return true;
    }

    /// How logger logs the message.
    fn log(&self, record: &log::Record) {
        // Skip certain logs.
        if match record.target() {
            "globset" => true,
            "mio::poll" => true,
            "tracing::span" | "tracing::span::active" => true,
            "handlebars::render" => record.level() > log::Level::Info,
            _ => {
                if record.target().starts_with("tower_http") && record.level() > log::Level::Warn {
                    true
                } else {
                    false
                }
            }
        } {
            return;
        }

        let mut prelude_len = 7;
        let module = record.module_path().unwrap_or("???");
        prelude_len += module.len();
        let now_string = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        prelude_len += now_string.len();

        print!(
            "{}[{}]{}: ",
            self.get_level_string_colored(record.level()),
            now_string.green(),
            record.module_path().unwrap_or("???").green().bold(),
        );
        for (i, line) in format!("{}", record.args()).split("\n").enumerate() {
            if line.is_empty() {
                continue;
            }
            if i > 0 {
                for _ in 0..prelude_len {
                    print!(" ");
                }
            }
            println!("{}", line);
        }
    }

    fn flush(&self) {}
}

/// Read embedded file as data.
pub fn read_embedded_data<Embed: RustEmbed>(path: impl AsRef<str>) -> Result<EmbeddedData> {
    match Embed::get(path.as_ref()) {
        Some(file) => Ok(EmbeddedData(file.data)),
        None => bail!("Unable to find file {}", path.as_ref()),
    }
}

/// Parse embedded file to text.
pub fn read_embedded_text<Embed: RustEmbed>(path: impl AsRef<str>) -> Result<String> {
    let data = read_embedded_data::<Embed>(path.as_ref())?;
    match std::str::from_utf8(&data) {
        Ok(file) => Ok(file.to_string()),
        _ => bail!("Unable to convert binary file {} to string", path.as_ref()),
    }
}

/// Read embedded toml.
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
