# Web dependency explanation

This owner implements `mgc why` for Web lockfiles. It reads legacy v2/v3 and canonical v4 graphs, validates v4 digest/policy before reporting, explicitly warns that signature trust is not evaluated, and presents reverse dependents with the existing stable sort and 20-row cap. It does not resolve or mutate dependencies. The old `core::shared::why` route remains exported for compatibility.

`error.rs` owns Web-only refusal diagnostics. `test/` contains the moved edge parser and dangling-v4-root regression sources plus typed error cases. They are not compiled or run in this structure pass.

# Giải thích dependency Web

Owner này triển khai `mgc why` cho lockfile Web. Nó đọc graph v2/v3 cũ và v4 canonical, xác minh digest/policy của v4 trước khi báo cáo, cảnh báo rõ rằng chưa đánh giá trust chữ ký, rồi hiển thị dependent theo thứ tự ổn định và giới hạn 20 dòng. Lệnh không resolve hoặc sửa dependency. Đường `core::shared::why` cũ tiếp tục được export để tương thích.

`error.rs` sở hữu diagnostic từ chối riêng Web. `test/` chứa source regression dangling-root v4, parser edge được chuyển và test lỗi typed. Chưa compile hoặc chạy chúng trong lượt cấu trúc này.
