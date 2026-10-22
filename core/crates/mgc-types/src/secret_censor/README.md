# Secret-censor owner

This owner exposes the compatibility helper `censor_secret`. It performs case-sensitive literal substring replacement, in this order: `token`, `password`, `secret`, then `key`. It does not parse configuration syntax and does not redact values following keys such as `token=...`; do not use it as a security-grade log or configuration redactor.

Owner này cung cấp helper tương thích `censor_secret`. Hàm thay thế chuỗi literal có phân biệt hoa thường theo thứ tự: `token`, `password`, `secret`, rồi `key`. Hàm không parse cú pháp cấu hình và không che giá trị phía sau khóa như `token=...`; không dùng nó như bộ redactor bảo mật cho log hay cấu hình.

The crate-root re-export preserves `mgc_types::censor_secret`. No error owner is needed because this transformation has no failure path. A structured redactor requires a separately approved format contract.

Export crate giữ nguyên đường dẫn `mgc_types::censor_secret`. Không cần owner lỗi vì phép biến đổi này không có nhánh thất bại. Redactor có cấu trúc cần một hợp đồng định dạng được duyệt riêng.
