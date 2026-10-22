# Windows Job process snapshot

This owner queries `JobObjectBasicProcessIdList`, owns the initial and maximum process-ID buffer capacities, retries changing process sets a fixed number of times, and returns typed errors when a complete snapshot cannot be obtained. The 20-attempt member-snapshot retry count stays shared with member capture; the 25 ms retry delay stays shared between this owner and the Windows command-line retry owner.

Owner này truy vấn `JobObjectBasicProcessIdList`, sở hữu sức chứa ban đầu và tối đa của buffer PID, retry tập process thay đổi đúng số lần hữu hạn và trả lỗi typed khi snapshot không thể hoàn chỉnh. Số lần retry snapshot 20 lần vẫn dùng chung với member capture; độ trễ 25 ms vẫn dùng chung giữa owner này và owner retry command line Windows.

`process_snapshot.rs` owns the native query and retry policy. `test/` checks an empty Job snapshot through a real Job Object. Test Windows: `cargo test -p mgc-exec empty_job_process_snapshot_is_empty`.
