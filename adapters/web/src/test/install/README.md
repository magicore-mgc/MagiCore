# Web install integration test owners

This owner groups the cross-module install tests by the behavior they verify: mutation policy, package layout, transaction boundaries, cache fallback, shared-store links, and extracted-root recovery. The tests stay in the original crate test scope so they retain access to the parent harness fixtures through `use super::*`. Each full `#[tokio::test]` item is preserved.

# Owner test tích hợp install Web

Owner này nhóm test install đi qua nhiều module theo hành vi được kiểm tra: mutation policy, package layout, transaction, cache fallback, shared-store link và recovery extracted-root. Test vẫn thuộc scope test crate ban đầu để dùng fixture của harness cha qua `use super::*`. Mỗi item `#[tokio::test]` được giữ nguyên.
