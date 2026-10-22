# Verified Job member capture

This owner opens only a PID from the private Job snapshot with limited-query access, verifies the process is still in that Job, and returns an owned handle that pins process identity through metadata capture. The invalid-PID exit race is rechecked against a fresh bounded Job snapshot; unverifiable membership fails closed.

Owner này chỉ mở PID lấy từ snapshot Job riêng bằng quyền query giới hạn, xác minh process vẫn thuộc Job rồi trả owned handle giữ danh tính tới khi chụp metadata. Race PID không hợp lệ được kiểm tra lại bằng snapshot Job mới có giới hạn; membership không xác minh được thì fail-closed.

`member_capture.rs` owns this security boundary. `test/` uses the current process, outside the fresh private Job, to verify membership mismatch fails closed. Test Windows: `cargo test -p mgc-exec member_capture_rejects_process_outside_private_job`.
