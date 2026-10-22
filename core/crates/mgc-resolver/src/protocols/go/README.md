# Go module protocol

The stable `protocols::go` facade for the native Go proxy engine. This feature has no template/runtime bypass; it resolves through the registry protocol.

| Owner | Responsibility |
|---|---|
| `go` | Protocol identity, proxy configuration, public artifact types |
| `http` | GOPROXY and sumdb requests, artifact downloads |
| `resolve` | Version selection, exact pins, graph expansion and integrity dispatch |
| `version` | Go version parsing and range operators |
| `cache` | Coordinate safety, Go path escaping and cache materialization |
| `integrity` | Go sumdb `h1:` directory hash |
| `manifest` | `go.mod` types and require/replace/exclude parsing |
| `metadata` | `.info` and `@latest` response validation |

All owners return the shared `mgc_types::MgError`/`MgResult` taxonomy. There is no duplicate Go error enum: network, integrity, invalid-version and parser errors keep their existing categories and messages.

## Tiếng Việt

Facade ổn định `protocols::go` cho engine Go proxy native; luồng resolve vẫn đi qua registry protocol, không chuyển sang template/runtime bypass. Bảng owner bên trên xác định từng nơi chịu trách nhiệm. Các owner dùng chung `mgc_types::MgError`/`MgResult`; không tạo enum lỗi Go trùng lặp. Loại lỗi và thông điệp hiện có được giữ nguyên. Unit test được đặt cạnh owner; HTTP integration tests vẫn ở `core/crates/mgc-resolver/tests/protocols_go.rs`.
