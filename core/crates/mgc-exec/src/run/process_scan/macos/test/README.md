# macOS process snapshot tests

Host-native check for the current PID identity query. It requires macOS and does not substitute for process-list, timeout, or process-tree E2E coverage.

Kiểm tra native trên macOS cho truy vấn identity PID hiện tại; không thay thế E2E liệt kê process, timeout hay cây process.

`mod.rs` only includes `macos.rs` so the test module keeps its original parent scope.
`mod.rs` chỉ include `macos.rs` để giữ nguyên scope cha của test module.
