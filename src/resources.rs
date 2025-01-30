//! Site resources.

use super::*;

/// Get media from resource data.
#[derive(RustEmbed)]
#[folder = "content/media"]
#[exclude = "*/*"]
#[include = "favicon.ico"]
struct EmbeddedFavicon;
pub async fn get_favicon(State(_site): State<SharedSite>) -> Vec<u8> {
    match util::read_embedded_data::<EmbeddedFavicon>("favicon.ico") {
        Ok(data) => data,
        Err(_) => {
            error!("Favicon missing!");
            vec![]
        }
    }
}

/// Get favicon from resource data.
#[derive(RustEmbed)]
#[folder = "content/media"]
#[exclude = "favicon.ico"]
struct EmbeddedMedia;
pub async fn get_media(Path(path): Path<String>, State(_site): State<SharedSite>) -> Vec<u8> {
    match util::read_embedded_data::<EmbeddedMedia>(&path) {
        Ok(data) => data,
        Err(_) => {
            error!("Asked for missing media {path}");
            vec![]
        }
    }
}

/// Get styles from resource data.
#[derive(RustEmbed)]
#[folder = "content/styles/"]
struct EmbeddedStyles;
pub async fn get_style(Path(path): Path<String>, State(_site): State<SharedSite>) -> Vec<u8> {
    match util::read_embedded_data::<EmbeddedStyles>(&path) {
        Ok(data) => data,
        Err(e) => {
            error!("Asked for invalid asset at style/{path}: {e}");
            vec![]
        }
    }
}

/// Get font from resource data.
#[derive(RustEmbed)]
#[folder = "content/fonts/"]
struct EmbeddedFonts;
pub async fn get_font(Path(path): Path<String>, State(_site): State<SharedSite>) -> Vec<u8> {
    match util::read_embedded_data::<EmbeddedFonts>(&path) {
        Ok(data) => data,
        Err(e) => {
            error!("Asked for invalid asset at fonts/{path}: {e}");
            vec![]
        }
    }
}
