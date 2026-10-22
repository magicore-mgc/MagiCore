# NuGet v3 protocol

Stable `protocols::nuget` facade. Owners split the registry path without changing its user-visible selection, integrity or installation behavior.

| Owner | Responsibility |
|---|---|
| `nuget` | Protocol identity, configuration and service-index setup |
| `http` | HTTP and flat-container URLs |
| `registration` | Resource discovery and paginated leaves |
| `resolve` | Version/dependency selection and SHA-512 verification |
| `cache` | global-packages extraction and sidecar |
| `framework` | TFM parsing and compatibility ranking |
| `version` | NuGet range semantics |
| `manifest` | PackageReference parsing/model |
| `xml` | Shared XML-like tag attribute/block scan |

Errors retain shared `MgError`/`MgResult` categories; this subsystem does not duplicate error types.

## Tiếng Việt

Facade ổn định `protocols::nuget`. Các owner chỉ tách trách nhiệm; giữ nguyên hành vi chọn version, toàn vẹn và cài đặt. Lỗi tiếp tục dùng `MgError`/`MgResult` chung, không tạo loại lỗi NuGet trùng lặp. Unit tests nằm sát owner; integration tests tiếp tục ở `core/crates/mgc-resolver/tests/protocols_nuget.rs`.
