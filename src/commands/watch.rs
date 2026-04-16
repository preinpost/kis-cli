use anyhow::Result;

use crate::client::KisClient;
use crate::ws;

pub async fn run(client: &KisClient, symbol: &str) -> Result<()> {
    ws::run(client.token_manager.clone(), symbol).await
}
