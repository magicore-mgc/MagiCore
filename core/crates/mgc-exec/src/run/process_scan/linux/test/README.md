# Linux process snapshot tests

Tests the Linux stat parser's PID/start-time identity, zombie classification, and the fail-closed rule for still-present versus vanished `/proc` directories.

Test identity PID/start-time, trạng thái zombie và quy tắc fail-closed giữa thư mục `/proc` còn tồn tại hoặc đã biến mất.

`mod.rs` only includes `linux.rs` so the test module keeps its original parent scope.
`mod.rs` chỉ include `linux.rs` để giữ nguyên scope cha của test module.
