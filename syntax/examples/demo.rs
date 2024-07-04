use anyhow::Result;
use tokio::time::sleep;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    sleep(Duration::from_micros(1)).await;
    Ok(())
}
