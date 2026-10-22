# Suspended primary thread

This owner finds the thread belonging to a newly spawned suspended child and resumes it exactly once. The caller must assign the child to the kill-on-close Job first; unexpected suspend counts remain typed failures.

Owner này tìm thread thuộc child mới spawn ở trạng thái suspend và resume đúng một lần. Caller phải gắn child vào Job kill-on-close trước; suspend count ngoài dự kiến vẫn là lỗi typed.

`primary_thread.rs` owns Toolhelp enumeration and resume ordering. `test/` covers the accepted and rejected prior suspend counts; native enumeration is covered by the Windows lifecycle E2E lane. Test Windows: `cargo test -p mgc-exec primary_thread_resume_requires_single_suspend`.
