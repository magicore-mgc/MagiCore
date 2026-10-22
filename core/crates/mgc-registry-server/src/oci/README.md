# OCI registry routes

## Responsibility

This owner implements the OCI Distribution API mounted at `/v2/*`: route parsing and authorization dispatch, blob read/upload, manifest read/write/validation, tags, and repository catalog. Its public API remains `crate::oci::routes()`.

## Trách nhiệm

Owner này triển khai OCI Distribution API tại `/v2/*`: phân giải route và phân quyền, đọc/upload blob, đọc/ghi/kiểm tra manifest, liệt kê tag và catalog repository. API công khai giữ nguyên `crate::oci::routes()`.

## Layout

- `mod.rs`: public facade only.
- `oci.rs`: child-owner declarations and route re-export.
- Each child folder owns one endpoint responsibility through a same-name source file and a small `mod.rs`.
- Nearest-owner source regressions are in each responsible `test/`; registered HTTP integration tests remain under the crate's `tests/` directory.

## Structure and compatibility

HTTP routes, authorization order, status codes, response headers/bodies, registry calls, and validation behavior are part of this module's compatibility boundary. Cross-owner calls use the explicit exports in `oci.rs` and the owner facades.

## Cấu trúc và tương thích

Route HTTP, thứ tự phân quyền, status code, header/body phản hồi, lời gọi registry và hành vi validation thuộc contract của module. Các owner gọi nhau qua export tường minh và facade riêng.
