# Trust policy input owner

`policy.rs` loads the machine-local trust-policy database and the committed project `[scripts]` policy. Database-open/read diagnostics continue through the existing shared trust error boundary because sibling trust commands use those factories. Project script-policy errors use the local pending error type.

Tests in `test/` cover unreadable and populated databases and malformed project policy. They are authored and source-mapped; execution is deferred.

# Owner input policy trust

`policy.rs` đọc trust-policy database cục bộ và policy `[scripts]` đã commit trong project. Lỗi mở/đọc database vẫn qua error boundary trust dùng chung vì lệnh trust khác cũng dùng các factory đó. Lỗi policy script project dùng error type cục bộ của pending.

Test trong `test/` kiểm tra database không đọc được, database có policy và project policy sai định dạng. Test đã có source và được ánh xạ; chưa chạy.
