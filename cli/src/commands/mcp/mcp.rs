//! MCP command coordinator.
//! Điều phối lệnh MCP và giữ API run công khai

use anyhow::Result;

pub async fn run() -> Result<()> {
    super::stdio::serve().await
}
