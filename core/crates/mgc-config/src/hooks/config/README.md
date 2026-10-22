# Config owner / Owner config

Owns the public `HooksConfig` value and TOML deserialization of each hook file. Path choice and merge ordering belong to separate owners. `mod.rs` only wires/re-exports; `config.rs` contains the implementation.

Sở hữu kiểu công khai `HooksConfig` và giải mã TOML cho từng file hook. Việc chọn path và thứ tự merge thuộc owner riêng. `mod.rs` chỉ nối/export; `config.rs` chứa implementation.

Preserve the stable `mgc_config::hooks` API and current `anyhow` error behavior. The nearest tests are in `test/` and remain `NOT_RUN` under the source-first gate.

Giữ API ổn định `mgc_config::hooks` và hành vi lỗi `anyhow` hiện tại. Test gần owner nằm trong `test/` và vẫn `NOT_RUN` theo source-first gate.
