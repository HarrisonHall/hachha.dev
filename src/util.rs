//! Utils.

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
