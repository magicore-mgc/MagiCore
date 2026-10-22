# Game adapter source / Mã nguồn Game adapter

## Purpose / Mục đích

This directory owns Game core implementation. `engine.rs` identifies the selected engine, `sbom.rs` bridges lockfiles to SBOM generation, and each subfolder owns one lifecycle or adapter responsibility.

Thư mục này chứa triển khai Game core. `engine.rs` nhận diện engine, `sbom.rs` nối lockfile với tạo SBOM; mỗi thư mục con phụ trách một phần lifecycle hoặc adapter.

## Errors and tests / Lỗi và kiểm thử

Production paths use shared `MgError`/`MgResult`; do not add local error taxonomies without a distinct domain invariant. Existing unit tests stay in the matching `test/` folders; public crate API tests are in [`adapters/game/tests/`](../tests/README.md).
