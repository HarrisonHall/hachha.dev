use actix_web::{get, web, App, HttpServer};
use log::*;
use rust_embed::RustEmbed;

use crate::site::Site;
use crate::util;

/// Embedded site favicon.
#[derive(RustEmbed)]
#[folder = "content/media"]
#[exclude = "*/*"]
#[include = "favicon.ico"]
struct EmbeddedFavicon;

/// Get site favicon.
#[get("/favicon.ico")]
async fn get_favicon(_site: web::Data<Site>) -> Vec<u8> {
    match util::read_embedded_data::<EmbeddedFavicon>("favicon.ico") {
        Ok(data) => data,
        Err(_) => {
            error!("Favicon missing!");
            vec![]
        }
    }
}

/// Embedded site media.
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
