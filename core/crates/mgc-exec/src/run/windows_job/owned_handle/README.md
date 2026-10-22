# Windows owned handle

This owner is the RAII boundary for valid Win32 handles used by the private Job Object flow. It owns exactly one `CloseHandle`, records close failures without panicking, and provides query-only image-path and process-exit reads while retaining handle identity.

Owner này là ranh giới RAII cho Win32 handle hợp lệ của luồng Job Object riêng. Nó thực hiện đúng một `CloseHandle`, ghi lỗi đóng mà không panic, và đọc image path/trạng thái thoát chỉ với quyền query trong khi giữ danh tính handle.

`owned_handle.rs` owns the handle type, query operations and maximum UTF-16 image-path buffer size; raw handles are visible only inside the Windows Job implementation. `test/` opens the current process with limited query access and checks metadata reads. Test Windows: `cargo test -p mgc-exec owned_handle_reads_current_process_identity`.
