# Swift registry integration-test owners

These integration tests compile in the existing `mgc-resolver` test crate and exercise the public resolver API. `support.rs` owns the mock HTTP server and hand-built archive fixture; lifecycle and fail-closed behavior have separate files. The Cargo target remains `tests/protocols_swift.rs`.

## Tiếng Việt

Các integration test này compile trong test crate hiện có của `mgc-resolver` để kiểm tra public resolver API. `support.rs` giữ mock HTTP server và fixture archive tự dựng; lifecycle và fail-closed được tách riêng. Cargo target tiếp tục là `tests/protocols_swift.rs`.

The `registry_url/` child is the crate-local unit-test owner for URL construction, registered from `registry/registry.rs`; it is separate from this public-API integration harness. The 15 relocated unit tests remain NOT_RUN in the source-first phase.

Folder con `registry_url/` là owner unit test trong crate cho việc ghép URL, được đăng ký từ `registry/registry.rs`; nó tách biệt với integration harness public API này. 15 unit test được chuyển vẫn NOT_RUN trong phase source-first.
