# Store command

## Purpose

Owns the `mgc store` CLI boundary; the facade preserves its public route while focused owners handle core routing, project identity, operations, and doctor safety.

## Mục đích

Sở hữu bề mặt CLI `mgc store`; facade giữ route public, các owner chuyên trách xử lý core, identity project, thao tác và an toàn doctor.

Each owner has a same-name Rust implementation, wiring-only `mod.rs`, and bilingual README. Existing private helper tests remain mounted from `cli/src/test/store_test.rs`.

Mỗi owner có implementation Rust cùng tên, `mod.rs` chỉ wiring và README song ngữ. Test helper private vẫn mount từ `cli/src/test/store_test.rs`.
