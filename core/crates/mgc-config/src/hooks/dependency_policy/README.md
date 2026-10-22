# Dependency Policy owner / Owner dependency policy

Recognizes dependency lifecycle verbs and refuses configured pre/post hooks before dependency mutations can begin. `mod.rs` only wires/re-exports; `dependency_policy.rs` contains the implementation.

Nhận diện động từ lifecycle dependency và từ chối hook pre/post đã cấu hình trước khi bắt đầu thao tác dependency. `mod.rs` chỉ nối/export; `dependency_policy.rs` chứa implementation.

Preserve the stable `mgc_config::hooks` API and current `anyhow` error behavior. The nearest tests are in `test/` and remain `NOT_RUN` under the source-first gate.

Giữ API ổn định `mgc_config::hooks` và hành vi lỗi `anyhow` hiện tại. Test gần owner nằm trong `test/` và vẫn `NOT_RUN` theo source-first gate.
