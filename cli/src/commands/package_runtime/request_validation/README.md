# Request Validation

Validate registry package requests before creating a workspace; reject oversized names, malformed scoped names, controls, URL/path delimiters and unsupported selectors.
Kiểm request registry trước khi tạo workspace; từ chối tên quá dài/sai, scoped name sai, control, dấu URL/path và selector không hỗ trợ.

## Files / File sở hữu

- `mod.rs` wires and re-exports this owner.
- `request_validation.rs`: `request_validation.rs` owns `parse_request`; `error.rs` owns typed invalid-request diagnostics. `mod.rs` exports the parser and compatibility factory.
- `test/request_validation.rs`: `test/request_validation.rs` checks valid names/selectors and traversal/URL/control rejection; `test/error.rs` checks typed safe-reason handling.
- `error.rs` and `test/error.rs`: typed failure and diagnostic contract where this owner emits a domain error.

`request_validation.rs sở hữu parse_request; error.rs giữ lỗi request typed. mod.rs export parser và factory.`

## Test and limits / Test và giới hạn

`test/request_validation.rs` checks valid names/selectors and traversal/URL/control rejection; `test/error.rs` checks typed safe-reason handling. Does not resolve versions or contact a registry; it validates syntax only.

test/request_validation.rs kiểm request hợp lệ và từ chối traversal/URL/control; test/error.rs kiểm reason an toàn và typed. Không resolve version hay gọi registry; chỉ kiểm cú pháp.

The registered unit command is `cargo test -p mgc --lib commands::package_runtime::`; it has not been run in this working session.
Lệnh unit đã đăng ký là `cargo test -p mgc --lib commands::package_runtime::`; chưa chạy trong phiên này.
