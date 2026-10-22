// Shared command route test — confirms capability command reaches its handler.
// Test route command dùng chung — xác nhận lệnh capabilities đi tới handler.
use super::dispatch_common;
use crate::dispatch::types::CommonCommand;

#[tokio::test]
async fn capabilities_command_runs_through_common_dispatch() {
    let result = dispatch_common(CommonCommand::Capabilities, None, false).await;
    assert!(result.is_ok(), "capabilities route failed: {result:?}");
}
