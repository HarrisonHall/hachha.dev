//! Main.

mod cache;
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
    let site = Site::new()?;
    site.serve().await?;
    Ok(())
}
