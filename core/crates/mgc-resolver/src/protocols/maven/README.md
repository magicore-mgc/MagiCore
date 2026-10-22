# Maven resolver protocol / Protocol resolver Maven

## Responsibility

This module implements native Maven Central resolution, POM graph extraction, checksum verification, and local Maven-layout materialization. Its public facade remains `mgc_resolver::protocols::maven`; adapters can continue using `MavenProtocol` and the existing POM parsing functions through their current paths.

## Structure and safety

- `registry/` owns repository configuration, HTTP helpers, parent traversal, and cache layout writes.
- `registry_protocol/` owns the `RegistryProtocol` implementation and artifact integrity checks.
- `pom_xml/`, `pom_dependencies/`, `pom_properties/`, `pom_coordinates/`, and `version_requirement/` each own one parsing/resolution concern.
- `types/` contains the shared `PomDependency` DTO.
- `test/` holds the existing Maven unit cases; integration cases remain under the crate `tests/` directory.

Errors use shared `MgError`/`MgResult`; malformed XML, unsafe cache coordinates, missing dependency versions, and missing/mismatched checksums remain errors. This move does not alter network selection or resolver behavior.

## Ghi chú tiếng Việt

Module này xử lý resolve Maven Central, đọc graph từ POM, xác minh checksum và ghi artifact theo layout Maven local. Facade công khai `mgc_resolver::protocols::maven` được giữ để adapter hiện tại không phải đổi đường import.

Mỗi folder con chỉ giữ một trách nhiệm như danh sách ở trên; lỗi tiếp tục dùng `MgError`/`MgResult` chung. XML lỗi, tọa độ cache không an toàn, version dependency thiếu và checksum thiếu/sai vẫn phải bị chặn. Đây là refactor cấu trúc, không thay đổi hành vi mạng hoặc resolve.
