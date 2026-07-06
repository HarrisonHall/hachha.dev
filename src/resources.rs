//! Site resources.

use super::*;

/// Get favicon from resource data.
pub async fn get_favicon(uri: Uri, State(site): State<Site>) -> impl axum::response::IntoResponse {
    let data = match site.packed_data().read_data("resources/media/favicon.ico") {
        Ok(data) => {
            EndpointHistoryOptions::default()
                .write(&site, uri.path())
                .await;
            data
        }
        Err(_) => {
            tracing::error!("Favicon missing!");
            util::EmbeddedData::empty()
        }
    };
    adjust_content_header("favicon.ico", data)
}

/// Get media from resource data.
pub async fn get_media(
    uri: Uri,
    Path(path): Path<String>,
    State(site): State<Site>,
) -> impl axum::response::IntoResponse {
    let path = format!("resources/media/{path}");
    let data = match site.packed_data().read_data(&path) {
        Ok(data) => {
            EndpointHistoryOptions::default()
                .write(&site, uri.path())
                .await;
            data
        }
        Err(_) => {
            EndpointHistoryOptions::builder()
                .valid(true)
                .build()
                .write(&site, uri.path())
                .await;
            util::EmbeddedData::empty()
        }
    };
    adjust_content_header(path, data)
}

/// Get styles from resource data.
pub async fn get_style(
    uri: Uri,
    Path(path): Path<String>,
    State(site): State<Site>,
) -> impl axum::response::IntoResponse {
    let path = format!("resources/styles/{path}");
    let data = match site.packed_data().read_data(&path) {
        Ok(data) => {
            EndpointHistoryOptions::default()
                .write(&site, uri.path())
                .await;
            data
        }
        Err(e) => {
            EndpointHistoryOptions::builder()
                .valid(true)
                .build()
                .write(&site, uri.path())
                .await;
            tracing::error!("Asked for invalid style at {path}: {e}");
            util::EmbeddedData::empty()
        }
    };
    adjust_content_header(path, data)
}

/// Get font from resource data.
pub async fn get_font(
    uri: Uri,
    Path(path): Path<String>,
    State(site): State<Site>,
) -> impl axum::response::IntoResponse {
    let path = format!("resources/fonts/{path}");
    let data = match site.packed_data().read_data(&path) {
        Ok(data) => {
            EndpointHistoryOptions::default()
                .write(&site, uri.path())
                .await;
            data
        }
        Err(e) => {
            EndpointHistoryOptions::builder()
                .valid(true)
                .build()
                .write(&site, uri.path())
                .await;
            tracing::error!("Asked for invalid asset at fonts/{path}: {e}");
            util::EmbeddedData::empty()
        }
    };
    adjust_content_header(path, data)
}

/// Get robots.txt
pub async fn get_robots_txt(
    uri: Uri,
    State(site): State<Site>,
) -> impl axum::response::IntoResponse {
    EndpointHistoryOptions::default()
        .write(&site, uri.path())
        .await;
    let data = match site.packed_data().read_data("resources/robots.txt") {
        Ok(data) => data,
        Err(_) => {
            tracing::error!("robots.txt missing!");
            util::EmbeddedData::empty()
        }
    };
    adjust_content_header("robots.txt", data)
}
