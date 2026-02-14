//! Site resources.

use super::*;

/// Get favicon from resource data.
pub async fn get_favicon(State(site): State<Site>) -> impl axum::response::IntoResponse {
    let data = match site.packed_data().read_data("media/favicon.ico") {
        Ok(data) => data,
        Err(_) => {
            tracing::error!("Favicon missing!");
            util::EmbeddedData::empty()
        }
    };
    adjust_content_header("favicon.ico", data)
}

/// Get media from resource data.
pub async fn get_media(
    Path(path): Path<String>,
    State(site): State<Site>,
) -> impl axum::response::IntoResponse {
    let path = format!("media/{path}");
    let data = match site.packed_data().read_data(&path) {
        Ok(data) => data,
        Err(_) => {
            tracing::error!("Asked for missing media {path}");
            util::EmbeddedData::empty()
        }
    };
    adjust_content_header(path, data)
}

/// Get styles from resource data.
pub async fn get_style(
    Path(path): Path<String>,
    State(site): State<Site>,
) -> impl axum::response::IntoResponse {
    // TODO - special handling for theme.css
    let path = format!("styles/{path}");
    let data = match site.packed_data().read_data(&path) {
        Ok(data) => data,
        Err(e) => {
            tracing::error!("Asked for invalid asset at style/{path}: {e}");
            util::EmbeddedData::empty()
        }
    };
    adjust_content_header(path, data)
}

/// Get font from resource data.
pub async fn get_font(
    Path(path): Path<String>,
    State(site): State<Site>,
) -> impl axum::response::IntoResponse {
    let path = format!("fonts/{path}");
    let data = match site.packed_data().read_data(&path) {
        Ok(data) => data,
        Err(e) => {
            tracing::error!("Asked for invalid asset at fonts/{path}: {e}");
            util::EmbeddedData::empty()
        }
    };
    adjust_content_header(path, data)
}

/// Get robots.txt
pub async fn get_robots_txt(State(site): State<Site>) -> impl axum::response::IntoResponse {
    let data = match site.packed_data().read_data("robots.txt") {
        Ok(data) => data,
        Err(_) => {
            tracing::error!("robots.txt missing!");
            util::EmbeddedData::empty()
        }
    };
    adjust_content_header("robots.txt", data)
}
