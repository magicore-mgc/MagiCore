# Hardware adapter tests

The integration test target is auto-discovered by Cargo. `integration.rs` preserves the registered test names, attributes, and order; behavior lives in `integration/` owners. Runtime verification is tracked separately.

Run this target with `cargo test -p mgc-hardware-adapter --test integration`. S324 documents the command but does not execute it.

# Test adapter Hardware

Cargo tự phát hiện integration test target. `integration.rs` giữ tên, attribute và thứ tự test; hành vi nằm trong các owner tại `integration/`. Runtime verification được theo dõi riêng.

Chạy target này bằng `cargo test -p mgc-hardware-adapter --test integration`. S324 ghi lệnh nhưng chưa thực thi.
