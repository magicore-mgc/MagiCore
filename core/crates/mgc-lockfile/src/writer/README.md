# Lockfile writer / Bộ ghi lockfile

The writer facade preserves `mgc_lockfile::writer::*`. `serialize_v3` enforces source completeness before producing v3 TOML: registry identity, imported-lockfile provenance with its source name, or a named delegated tool. `mutation_policy` protects signed and schema-mismatched files; `legacy_signing` owns the sidecar signer; `v4_finalization` computes and round-trips canonical v4 bytes. `mod.rs` only wires and exports.

Facade này giữ `mgc_lockfile::writer::*`. `serialize_v3` kiểm tra source đầy đủ trước khi sinh TOML v3: registry identity, provenance lockfile import kèm tên nguồn, hoặc tool ủy thác có tên. `mutation_policy` bảo vệ file đã ký và schema không khớp; `legacy_signing` sở hữu ký sidecar; `v4_finalization` tính digest và round-trip bytes v4 canonical. `mod.rs` chỉ nối và export.

V3 import records tagged `Other` remain exempt for compatibility. The v3 gate does not validate v4 source-table references; that is tracked separately.

Record import v3 có tag `Other` tiếp tục được miễn để tương thích. Gate v3 không xác minh foreign key bảng source của v4; việc đó được theo dõi riêng.
