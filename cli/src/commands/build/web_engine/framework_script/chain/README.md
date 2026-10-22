# Framework command chain / Chuỗi lệnh framework

Parses the complete `&&` chain, gates each runtime, maps every command first and refuses a partial/unsupported chain before any launch. It does not invoke a shell.

Parse toàn bộ chuỗi `&&`, kiểm tra từng runtime, map tất cả lệnh trước và từ chối chain không hỗ trợ trước khi chạy lệnh nào. Không gọi shell.

Errors from the shared allowlist parser and script mapper retain their source; no separate chain error domain is introduced. Tests remain in `cli/src/test/build_test.rs` for the deferred test phase.

Lỗi từ allowlist parser và script mapper dùng chung giữ source; không tạo error domain chain riêng. Test hiện ở `cli/src/test/build_test.rs`, chờ pha test.
