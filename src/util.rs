use handlebars::handlebars_helper;
use rust_embed::RustEmbed;
use log::*;

use serde_json::json;
use serde_yaml;
use pulldown_cmark::{Parser, Options, html};


pub fn read_embedded_text<Embed: RustEmbed>(path: &str) -> Result<String, String> {
    match Embed::get(path) {
        Some(file) => {
            match std::str::from_utf8(&file.data) {
                Ok(file) => Ok(file.to_owned()),
                _ => Err(format!("Unable to convert binary file {} to string", path)),
            }
        },
        None => Err(format!("Unable to find file {}", path)),
    }
}

pub fn read_embedded_data<Embed: RustEmbed>(path: &str) -> Result<Vec<u8>, String> {
    match Embed::get(path) {
        Some(file) => {
            Ok(file.data.to_vec())
        },
        None => Err(format!("Unable to find file {}", path)),
    }
}

pub fn read_yaml_to_json(yaml: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    Ok(serde_yaml::from_str::<serde_json::Value>(yaml)?)
}

/*
pub fn markdown_helper(h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output)
    -> Result<(), RenderError> {
let markdown_text_var =
try!(h.param(0)
.ok_or_else(|| RenderError::new("Param not found for helper \"markdown\"")));
let markdown_text = markdown_text_var.value().render();
let html_string = render_html(markdown_text);
try!(out.write(&html_string));
Ok(())
}
 */

handlebars_helper!(markdown_helper: |content: String| {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&content, options);

    // Write to String buffer.
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    info!("Rendered as `{}`", html_output);
    html_output
});