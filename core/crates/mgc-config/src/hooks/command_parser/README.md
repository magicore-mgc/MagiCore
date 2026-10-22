# Command Parser owner / Owner command parser

Converts a configured command to argv and rejects the existing shell control/metacharacter set. It never invokes a shell. `mod.rs` only wires/re-exports; `command_parser.rs` contains the implementation.

Chuyển command cấu hình thành argv và từ chối tập shell control/metacharacter hiện tại. Không gọi shell. `mod.rs` chỉ nối/export; `command_parser.rs` chứa implementation.

Preserve the stable `mgc_config::hooks` API and current `anyhow` error behavior. The nearest tests are in `test/` and remain `NOT_RUN` under the source-first gate.

Giữ API ổn định `mgc_config::hooks` và hành vi lỗi `anyhow` hiện tại. Test gần owner nằm trong `test/` và vẫn `NOT_RUN` theo source-first gate.
