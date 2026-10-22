# Native registry construction

This owner builds the native Rust, Python, Go, Maven, and NuGet protocol objects used by both project resolution and single-package `add`. Language-specific constructors live in same-named files; `metadata.rs` owns the ecosystem tag and operation-specific registry identity; `native_registry.rs` owns the shared resolver-engine call. Web/TypeScript continues through its existing Web adapter.

The Rust lock registry identifier intentionally remains operation-specific: project resolve writes the sparse-index identity, while single-package `add` retains the existing crates.io identity. Do not normalize the two without a separately approved behavior change.

## Phạm vi

Owner này dựng protocol native Rust, Python, Go, Maven và NuGet dùng chung cho resolve dự án và `add` một package. Constructor theo ngôn ngữ nằm trong file cùng tên; `metadata.rs` sở hữu ecosystem tag và registry identity theo thao tác; `native_registry.rs` sở hữu lời gọi engine dùng chung. TypeScript/Web tiếp tục qua Web adapter hiện tại.

Registry identity Rust trong lock cố ý khác theo thao tác: resolve dự án ghi sparse-index, còn `add` một package giữ identity crates.io hiện tại. Không tự chuẩn hóa hai giá trị nếu chưa có thay đổi hành vi được duyệt riêng.
