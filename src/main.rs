use anyhow::Result;
use rasto::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    server().await?;
    Ok(())
}
