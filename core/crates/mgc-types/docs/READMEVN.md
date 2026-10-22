# mgc-types

<!-- English module guide: purpose, source map, and review boundaries. -->
<!-- Hướng dẫn tiếng Việt về chức năng, source và ranh giới review của module. -->

## Chức năng

Định nghĩa contract/kiểu dùng chung: adapter/capability, ecosystem, identity package/version, manifest, model publish/patch, lỗi/result, JSONC và helper thay chuỗi literal cũ. Helper này không che giá trị cấu hình và không đạt mức redactor bảo mật. Thay đổi public có thể ảnh hưởng đa số crate; cần review caller và tương thích serialization/API.

## Sơ đồ source

- [`lib.rs`](../src/lib.rs)
- [`adapter/`](../src/adapter/README.md)
- [`capabilities/`](../src/capabilities/README.md)
- [`ecosystem/`](../src/ecosystem/README.md) — enum dùng chung, ánh xạ chuỗi và test gần owner.
- [`error/`](../src/error/README.md)
- [`jsonc/`](../src/jsonc/README.md)
- [`manifest/`](../src/manifest/README.md) — model Manifest dùng chung và test sát owner.
- [`package/`](../src/package/README.md)
- [`patch/`](../src/patch/README.md) — spec patch cấu hình và record patch lockfile.
- [`publish/`](../src/publish/README.md) — contract cấu hình request và kết quả publish, kèm test gần owner.
- [`secret_censor/`](../src/secret_censor/README.md) — helper literal cũ, không phải redactor an toàn.
- [`version/`](../src/version/README.md)

## Bàn giao và review

Phân tích caller trước khi sửa trait/enum public hoặc identity. Kiểm tra serialization, equality/order/hash, chuỗi ecosystem ownership và core parity.

## Xác minh

Sau khi sửa code, chạy test crate từ repo root bằng `cargo test -p mgc-types`. Với thay đổi chỉ có tài liệu, kiểm tra link, ngôn ngữ và Git ignore thay vì chạy test workspace không liên quan.
