# Windows Job lifecycle

This owner creates the kill-on-close Job Object, assigns each suspended child, then delegates the one-time primary-thread resume. It also owns explicit termination and delegates bounded member enumeration/capture to their own owners.

Owner này tạo Job Object kill-on-close, gắn child đang suspend rồi mới giao owner primary-thread resume đúng một lần. Nó cũng sở hữu lệnh terminate tường minh và giao truy vấn/chụp member có giới hạn cho các owner tương ứng.

`job_lifecycle.rs` preserves assign-before-resume ordering and stable `ProcessTreeGuard` methods. `test/` creates and terminates an empty Job without spawning a child. Test Windows: `cargo test -p mgc-exec empty_job_can_be_terminated`.
