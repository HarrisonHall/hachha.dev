//! Main.

mod cache;
mod db;
mod pages;
mod prelude;
mod resources;
mod site;
mod theme;
mod util;

use prelude::internal::*;
use prelude::*;

/// Server entry-point.
#[tokio::main]
async fn main() -> Result<()> {
    // Build/parse site and serve.
    let site = Site::new().await?;
    site.serve().await?;
    Ok(())
}
