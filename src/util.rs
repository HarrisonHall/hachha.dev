//! Utils.

use super::*;

use handlebars::handlebars_helper;
use markdown;
use rust_embed::RustEmbed;

const MD_RENDERING_ERROR: &str = "<p>Unable to render markdown :(</p>";

/// Parse embedded file to text.
pub fn read_embedded_text<Embed: RustEmbed>(path: &str) -> Result<String> {
    match Embed::get(path) {
        Some(file) => match std::str::from_utf8(&file.data) {
            Ok(file) => Ok(file.to_owned()),
            _ => bail!("Unable to convert binary file {} to string", path),
        },
        None => bail!("Unable to find file {}", path),
    }
}

/// Read embedded file as data.
pub fn read_embedded_data<Embed: RustEmbed>(path: &str) -> Result<Vec<u8>> {
    match Embed::get(path) {
        Some(file) => Ok(file.data.to_vec()),
        None => bail!("Unable to find file {}", path),
    }
}

/// Read embedded toml.
pub fn read_embedded_toml<T: DeserializeOwned, Embed: RustEmbed>(path: &str) -> Result<T> {
    match Embed::get(path) {
        Some(file) => match std::str::from_utf8(&file.data) {
            Ok(file) => toml::from_str::<T>(file)
                .map_err(|e| anyhow!("read_embedded_toml error for {}: {e}", path)),
            _ => bail!("Unable to convert binary file {} to string", path),
        },
        None => bail!("Unable to find file {}", path),
    }
}

// /// Serialize to json.
pub fn to_json<T: Serialize>(value: &T) -> Result<serde_json::Value> {
    Ok(serde_json::to_value(value)?)
}

handlebars_helper!(markdown_helper: |content: String| {
    let mut md_options = markdown::Options::gfm();
    md_options.compile.allow_dangerous_html = true;
    md_options.compile.allow_dangerous_protocol = true;
    let mut compiled_markdown = match markdown::to_html_with_options(&content, &md_options) {
        Ok(compiled_markdown) => compiled_markdown,
        Err(e) => {
            error!("Markdown rendering error: {}", e);
            MD_RENDERING_ERROR.to_string()
        }
    };
    compiled_markdown = compiled_markdown.replace("<pre>", "<pre class=\"code\">");  // Replace code blocks
    compiled_markdown
});

/// Merge json values.
pub fn merge_json(a: &mut serde_json::Value, b: &serde_json::Value) {
    match (a, &b) {
        (a @ &mut serde_json::Value::Object(_), &serde_json::Value::Object(b)) => {
            let a = a.as_object_mut().expect("Unable to merge json.");
            for (k, v) in b {
                merge_json(a.entry(k).or_insert(serde_json::Value::Null), &v);
            }
        }
        (a, b) => *a = (*b).clone(),
    }
}
