# Tests for `supervision`

The Unix contract test runs a real long-lived executable through the MagiCore project-binary gateway and verifies timeout handling. Platform process-tree and cancellation integration coverage remains in the crate's existing OS-specific test suites.

Test contract Unix chạy executable thật qua gateway project-binary của MagiCore và xác minh timeout. Coverage process-tree/cancellation theo nền tảng tiếp tục ở integration suite OS hiện có.

`mod.rs` only includes `supervision.rs` so the test module keeps its original parent scope.
`mod.rs` chỉ include `supervision.rs` để giữ nguyên scope cha của test module.
