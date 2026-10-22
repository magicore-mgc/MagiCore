# Manifest owner / Owner manifest

This folder owns the stable `mgc_pack::manifest` API used to sanitize package metadata before publish. `mod.rs` is only the public facade; behavior and data types live in same-name owners.

Folder này quản lý API `mgc_pack::manifest` ổn định để làm sạch metadata trước khi publish. `mod.rs` chỉ là facade công khai; xử lý và kiểu dữ liệu nằm trong các owner cùng tên.

| Owner | Responsibility | Trách nhiệm |
|---|---|---|
| `types/` | Public `Sanitized` result shape | Cấu trúc kết quả công khai `Sanitized` |
| `sanitize/` | Validate, filter and construct exportable JSON | Kiểm tra, lọc và tạo JSON có thể publish |
| `metadata_identity/` | Resolve empty fallbacks and non-empty overrides | Chọn fallback và override name/version |
| `dependency_fields/` | Preserve only the four dependency maps for publish body | Giữ đúng bốn map dependency cho publish body |
| `error/` | Local typed errors with stable display strings | Lỗi typed cục bộ với chuỗi hiển thị ổn định |

The external `tests/manifest.rs` target remains the public API integration contract. Owner tests stay beside the behavior and are registered as separate files. Runtime verification is pending.

Target `tests/manifest.rs` bên ngoài vẫn là integration contract cho API công khai. Test owner được đặt cạnh xử lý và đăng ký bằng file riêng. Runtime verification còn chờ.
