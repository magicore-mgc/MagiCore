# Tests for `spawn`

Tests here pin contracts owned by the spawn pipeline: dry-run no-spawn behavior, typed error sources, and combined execution/Windows handle-cleanup reporting. Tests are authored but NOT_RUN until structural refactoring is complete; use crate integration tests for real process, timeout, process-tree, environment, and audit behavior.

Các test tại đây khóa contract của luồng spawn: dry-run không spawn, source lỗi typed và báo lỗi execution/cleanup handle Windows đồng thời. Test đã viết nhưng NOT_RUN tới khi refactor cấu trúc xong; integration test crate kiểm tra process con thật, timeout, process tree, environment và audit.

`mod.rs` only includes `spawn.rs` so the test module keeps its original parent scope.
`mod.rs` chỉ include `spawn.rs` để giữ nguyên scope cha của test module.
