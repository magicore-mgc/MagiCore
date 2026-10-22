# Execution owner / Owner execution

Loads the merged config, enforces the dependency gate and routes each command through `mgc_exec` with the existing `TestRunner` scope. `mod.rs` only wires/re-exports; `execution.rs` contains the implementation.

Đọc config đã merge, áp dụng dependency gate và chạy từng lệnh qua `mgc_exec` với scope `TestRunner` hiện tại. `mod.rs` chỉ nối/export; `execution.rs` chứa implementation.

Preserve the stable `mgc_config::hooks` API and current `anyhow` error behavior. The nearest tests are in `test/` and remain `NOT_RUN` under the source-first gate.

Giữ API ổn định `mgc_config::hooks` và hành vi lỗi `anyhow` hiện tại. Test gần owner nằm trong `test/` và vẫn `NOT_RUN` theo source-first gate.
