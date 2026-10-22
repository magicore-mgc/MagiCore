# Context identity errors / Lỗi identity của context

`error.rs` owns the typed core-override conflict. Its `Display` stays byte-for-byte compatible with the established English CLI message; other config and ecosystem errors continue to propagate from their existing owners.

`error.rs` sở hữu lỗi typed khi `--core` xung đột. `Display` giữ nguyên nội dung tiếng Anh hiện hành; lỗi config và ecosystem khác tiếp tục được propagate từ owner cũ.

Nearest tests verify stable display text and downcasting. Test source is not yet run.

Test gần owner kiểm tra thông báo ổn định và khả năng downcast. Test chưa chạy.
