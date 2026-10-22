# Package bin materialization / Tạo bin cho package

Reads each affected package's declared binary entries, skips unsafe or missing targets, checks the canonical target stays within its package, and delegates link creation to the platform owner. Warning text and traversal order are compatibility behavior.

Đọc khai báo binary của từng package affected, bỏ qua target nguy hiểm hoặc không tồn tại, kiểm tra canonical target nằm trong package rồi giao tạo link cho owner nền tảng. Nội dung warning và thứ tự duyệt là hành vi tương thích cần giữ.

Manifest and link failures preserve the original `MgError` category and display text. Direct target-containment tests are absent at baseline and remain pending; do not call these branches runtime-verified. `cargo test -p mgc-web-adapter` is the later crate-level command (not run here).

Lỗi manifest/link giữ category và display text `MgError` gốc. Baseline chưa có test trực tiếp kiểm tra target containment; chưa được xem nhánh này là runtime-verified. `cargo test -p mgc-web-adapter` là lệnh crate-level cho phase sau (chưa chạy ở đây).
