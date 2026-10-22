# Python + Rust audit / Audit Python + Rust

Builds the shared audit plan with Python first and Rust second, adding only constructors that return a scan step. The dispatcher decides when this owner is selected.

Tạo plan audit dùng chung với Python trước, Rust sau; chỉ thêm constructor có scan step. Dispatcher quyết định khi nào gọi owner này.

Do not change ordering or silently convert an absent step into a clean result. Direct polyglot coverage remains pending; no tests ran in this source-only slice.

Không đổi thứ tự và không biến step thiếu thành kết quả sạch im lặng. Test polyglot trực tiếp còn pending; slice source-only này chưa chạy test.
