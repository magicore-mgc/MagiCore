# Go + Rust audit / Audit Go + Rust

Builds the shared audit plan with Go first and Rust second, adding only constructors that return a scan step. The dispatcher keeps the existing `Cargo.toml` selection condition.

Tạo plan audit dùng chung với Go trước, Rust sau; chỉ thêm constructor có scan step. Dispatcher giữ điều kiện chọn `Cargo.toml` hiện tại.

Do not change ordering or silently convert an absent step into a clean result. Direct polyglot coverage remains pending; no tests ran in this source-only slice.

Không đổi thứ tự và không biến step thiếu thành kết quả sạch im lặng. Test polyglot trực tiếp còn pending; slice source-only này chưa chạy test.
