# Lib manifest tests / Test manifest Lib

This folder owns the unit tests registered by `adapters/lib/src/manifest/mod.rs`. It keeps parser and writer behavior checks close to the Lib manifest implementation. `mod.rs` only declares child modules; `support.rs` provides shared private imports and the Go `go.mod` fixture.

Folder này sở hữu unit test được đăng ký trong `adapters/lib/src/manifest/mod.rs`, đặt kiểm tra parser và writer gần implementation manifest Lib. `mod.rs` chỉ khai báo module con; `support.rs` cung cấp import private dùng chung và fixture Go `go.mod`.

| Owner file | Responsibility |
|---|---|
| `format_detection.rs` | Native format ownership, Java lane selection, and .NET root project selection |
| `maven_parser.rs` | POM dependency resolution, XML edge cases, and fail-closed parsing |
| `dotnet_manifest.rs` | Root project ambiguity and unsupported PackageReference semantics |
| `writer_safety.rs` | Symlink refusal and atomic Maven mode preservation |
| `python_manifest.rs` | PEP 621/508/735 ownership and unsupported dependency metadata |
| `go_mod_parser.rs` | `require`, `replace`, `exclude`, and indirect-version parsing |
| `cargo_python_roundtrip.rs` | Cargo and Python dependency version round trips |
| `go_mod_roundtrip.rs` | Native Go writer and parser round trip |
| `maven_dotnet_roundtrip.rs` | Maven/.NET writes, metadata preservation, and parsing back |
| `manifest_pruning.rs` | Removed Maven/.NET dependency pruning |
| `support.rs` | Shared test imports and `write_go_mod` fixture |

All 37 moved test bodies and attributes are source-preserved. They remain `NOT_RUN`; this owner makes no compile or runtime claim.

Đã giữ nguyên source của 37 test được chuyển. Chúng vẫn `NOT_RUN`; folder này không tuyên bố compile hoặc runtime đã đạt.
