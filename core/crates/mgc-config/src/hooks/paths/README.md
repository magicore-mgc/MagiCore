# Paths owner / Owner paths

Builds the project-local path first and the user-global path second. It performs no file reads or config parsing. `mod.rs` only wires/re-exports; `paths.rs` contains the implementation.

Tạo path của dự án trước, path toàn user sau. Owner này không đọc file hoặc parse cấu hình. `mod.rs` chỉ nối/export; `paths.rs` chứa implementation.

Preserve the stable `mgc_config::hooks` API and current `anyhow` error behavior. The nearest tests are in `test/` and remain `NOT_RUN` under the source-first gate.

Giữ API ổn định `mgc_config::hooks` và hành vi lỗi `anyhow` hiện tại. Test gần owner nằm trong `test/` và vẫn `NOT_RUN` theo source-first gate.
