# Deferred handle-close failures

This owner stores `CloseHandle` failures produced by `WindowsOwnedHandle::drop` in a bounded thread-local ledger. `take_deferred_handle_close_error` drains that ledger once; at most 32 native error details are retained and later failures are counted.

Owner này lưu lỗi `CloseHandle` do `WindowsOwnedHandle::drop` tạo ra trong ledger thread-local có giới hạn. `take_deferred_handle_close_error` lấy và xóa ledger một lần; giữ tối đa 32 chi tiết mã lỗi, các lỗi sau chỉ tăng bộ đếm.

`deferred_handle_close.rs` contains behavior, `mod.rs` exports the drain function, and `test/` contains the moved owner-local bounded/drain test. Windows-only test: `cargo test -p mgc-exec deferred_handle_close_failures_are_thread_local_and_drained_as_one_error`.
