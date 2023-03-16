use std::net::SocketAddr;
use std::path::PathBuf;

use axum::{
    routing::{get, post},
    Json, Router
};
use clap::{Parser, Subcommand};
use handlebars::Handlebars;
use rust_embed::RustEmbed;
use serde_json::json;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    #[arg(short, long, value_name = "PORT", default_value_t = 8080)]
    port: u16,
    #[arg(short, long, value_name = "LOG_PATH")]
    log: Option<String>,
    #[arg(short, long, value_name = "DIRECTORY")]
    content: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    println!("nfra framework");

    let args = CliArgs::parse();

    let mut app = Router::new();
    app = app.route("/", get(root));

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


#[derive(RustEmbed)]
#[folder = "content/"]
#[exclude = "*.md"]
pub struct Content;

async fn root() -> String {
    let mut reg = Handlebars::new();
    return reg.render_template(
        std::str::from_utf8(&Content::get("pages/index.html").unwrap().data).unwrap(), 
        &json!({})
    ).unwrap();
}