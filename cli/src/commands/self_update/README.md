# self_update / Cập nhật CLI

Owns release identity, provenance, bounded transfer, integrity checks, safe archive staging and rollback-protected binary replacement.
Sở hữu nhận diện release, provenance, tải có giới hạn, kiểm tra integrity, staging archive an toàn và thay binary có rollback.

## Files / Tệp

- `mod.rs` — stable command facade.
- `self_update/` — ordered lifecycle coordinator.
- `version/`, `platform/`, `types/`, `provenance/` — identity and trust policy.
- `download/`, `checksum/`, `manifest/` — transport and integrity.
- `archive/`, `install/` — safe staging and replacement.
- `../test/self_update.rs` — existing test source kept unchanged and registered through the facade.
