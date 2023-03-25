use axum::extract::{Path, State};
use log::*;
use rust_embed::RustEmbed;

use crate::site::SharedSite;
use crate::util;

#[derive(RustEmbed)]
#[folder = "content/media"]
#[exclude = "*/*"]
#[include = "favicon.ico"]
struct EmbeddedFavicon;
pub async fn get_favicon<'a>(State(_site): State<SharedSite<'a>>) -> Vec<u8> {
    match util::read_embedded_data::<EmbeddedFavicon>("favicon.ico") {
        Ok(data) => data,
        Err(_) => {
            error!("Favicon missing!");
            vec![]
        }
    }
}

#[derive(RustEmbed)]
#[folder = "content/media"]
#[exclude = "favicon.ico"]
struct EmbeddedMedia;
pub async fn get_media<'a>(
    Path(path): Path<String>,
    State(_site): State<SharedSite<'a>>,
) -> Vec<u8> {
    match util::read_embedded_data::<EmbeddedMedia>(&path) {
        Ok(data) => data,
        Err(_) => {
            error!("Asked for missing media {path}");
            vec![]
        }
    }
}

#[derive(RustEmbed)]
#[folder = "content/styles/"]
struct EmbeddedStyles;
pub async fn get_style<'a>(
    Path(path): Path<String>,
    State(_site): State<SharedSite<'a>>,
) -> Vec<u8> {
    match util::read_embedded_data::<EmbeddedStyles>(&path) {
        Ok(data) => data,
        Err(e) => {
            error!("Asked for invalid asset at style/{path}: {e}");
            vec![]
        }
    }
}

#[derive(RustEmbed)]
#[folder = "content/fonts/"]
struct EmbeddedFonts;
pub async fn get_font<'a>(
    Path(path): Path<String>,
    State(_site): State<SharedSite<'a>>,
) -> Vec<u8> {
    match util::read_embedded_data::<EmbeddedFonts>(&path) {
        Ok(data) => data,
        Err(e) => {
            error!("Asked for invalid asset at fonts/{path}: {e}");
            vec![]
        }
    }
}
