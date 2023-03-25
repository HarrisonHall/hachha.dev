use handlebars::handlebars_helper;
use markdown;
use rust_embed::RustEmbed;
use serde_yaml;

pub fn read_embedded_text<Embed: RustEmbed>(path: &str) -> Result<String, String> {
    match Embed::get(path) {
        Some(file) => match std::str::from_utf8(&file.data) {
            Ok(file) => Ok(file.to_owned()),
            _ => Err(format!("Unable to convert binary file {} to string", path)),
        },
        None => Err(format!("Unable to find file {}", path)),
    }
}

pub fn read_embedded_data<Embed: RustEmbed>(path: &str) -> Result<Vec<u8>, String> {
    match Embed::get(path) {
        Some(file) => Ok(file.data.to_vec()),
        None => Err(format!("Unable to find file {}", path)),
    }
}

pub fn read_yaml_to_json(yaml: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    Ok(serde_yaml::from_str::<serde_json::Value>(yaml)?)
}

handlebars_helper!(markdown_helper: |content: String| {
    let md_options = markdown::Options::gfm();
    let mut compiled_markdown = markdown::to_html_with_options(&content, &md_options).unwrap();  // TODO - remove unwrap
    compiled_markdown = compiled_markdown.replace("<pre>", "<pre class=\"code\">");  // Replace code blocks
    compiled_markdown
});

pub fn merge_json(a: &mut serde_json::Value, b: &serde_json::Value) {
    match (a, &b) {
        (a @ &mut serde_json::Value::Object(_), &serde_json::Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge_json(a.entry(k).or_insert(serde_json::Value::Null), &v);
            }
        }
        (a, b) => *a = (*b).clone(),
    }
}
