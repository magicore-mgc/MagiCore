# Shutdown owner tests

`error.rs` verifies Unix/Windows handler registration variants retain the operation and native error code in their source chain. It does not install or replace process-global OS signal handlers.

# Test owner shutdown

`error.rs` xác minh lỗi đăng ký handler Unix/Windows giữ thao tác và mã lỗi native trong source chain. Test không cài hoặc thay handler signal toàn process.
