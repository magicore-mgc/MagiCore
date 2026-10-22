# Lifecycle package manifest owner

`package_json.rs` validates the package manifest root, the `scripts` object, and lifecycle-hook value types, then records the manifest's package name/version. It does not perform filesystem traversal or unbounded reads.

# Owner manifest lifecycle package

`package_json.rs` kiểm tra root manifest, object `scripts` và kiểu giá trị hook lifecycle, rồi ghi nhận tên/phiên bản trong manifest. Owner này không duyệt filesystem và không đọc file không giới hạn.
