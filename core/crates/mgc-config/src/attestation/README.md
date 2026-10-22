# Project core attestation / Chứng thực core project

This module binds the canonical core identity and exact `mgc.toml` digest to an Ed25519 key stored outside the project. `mod.rs` preserves all prior `mgc_config::attestation::*` public routes and only wires/re-exports the ten behavior owners.

Module này ràng buộc core identity canonical và digest chính xác của `mgc.toml` với key Ed25519 lưu ngoài project. `mod.rs` giữ mọi public path `mgc_config::attestation::*` cũ và chỉ nối/export mười owner hành vi.

The owners separate record types, enforcement mode, message/digest, project slot/config reads, keyring loading, attestation orchestration, rotation, signing/storage, verification and policy enforcement. No trust decision or error contract is intentionally changed; `anyhow` and `AttestationStatus` remain the boundaries.

Các owner tách type record, mode enforcement, message/digest, slot/đọc config, keyring, điều phối attest, rotation, ký/lưu, verify và enforcement policy. Không chủ ý đổi trust decision/error contract; giữ ranh giới `anyhow` và `AttestationStatus`.

Cross-operation coverage stays in `core/crates/mgc-config/tests/core_attestation.rs`; owner-local cases are in each `test/` folder. They are authored/wired but NOT_RUN until the source-first repository phase and storage gate finish.

Coverage xuyên operation tiếp tục ở `core/crates/mgc-config/tests/core_attestation.rs`; test từng owner nằm trong `test/`. Các test đã viết/nối nhưng NOT_RUN tới khi hoàn tất phase refactor source và storage gate.
