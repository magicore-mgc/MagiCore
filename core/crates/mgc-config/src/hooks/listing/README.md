# Listing owner / Owner listing

Returns the hook event map after the shared ordered merge; it does not start processes. `mod.rs` only wires/re-exports; `listing.rs` contains the implementation.

Trả về map event hook sau khi dùng chung việc merge có thứ tự; không chạy process. `mod.rs` chỉ nối/export; `listing.rs` chứa implementation.

Preserve the stable `mgc_config::hooks` API and current `anyhow` error behavior. The nearest tests are in `test/` and remain `NOT_RUN` under the source-first gate.

Giữ API ổn định `mgc_config::hooks` và hành vi lỗi `anyhow` hiện tại. Test gần owner nằm trong `test/` và vẫn `NOT_RUN` theo source-first gate.
