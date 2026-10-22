# Merge owner / Owner merge

Loads configured files in `hooks_paths` order and appends command lists for duplicate event keys. It preserves the observed project-then-user order. `mod.rs` only wires/re-exports; `merge.rs` contains the implementation.

Đọc file theo thứ tự `hooks_paths` và nối danh sách command ở các event trùng. Giữ thứ tự dự án rồi user đang có. `mod.rs` chỉ nối/export; `merge.rs` chứa implementation.

Preserve the stable `mgc_config::hooks` API and current `anyhow` error behavior. The nearest tests are in `test/` and remain `NOT_RUN` under the source-first gate.

Giữ API ổn định `mgc_config::hooks` và hành vi lỗi `anyhow` hiện tại. Test gần owner nằm trong `test/` và vẫn `NOT_RUN` theo source-first gate.
