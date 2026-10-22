# package_adapter

## Responsibility

Own the Lib `PackageAdapter` boundary. `package_adapter.rs` contains only the trait implementation and forwards each responsibility to a focused sibling owner.

| Owner | Responsibility |
| --- | --- |
| `capability_profile/` | Public capability arrays and per-language claim selection |
| `manifest_identity/` | Stable language/manifest identity |
| `age_gate/` | Project age-gate policy hooks |
| `add_preparation/` | Native resolve-first add preparation |
| `manifest_parse/` | Read-only language manifest dispatch |
| `installed_inventory/` | Verified installed-package inventory |
| `web_preferences/` | Forward Web-only preference settings |

The parent `mod.rs` wires these owners and re-exports their private helpers only within the adapter. Errors continue through shared `MgError`/`MgResult`; no parallel error layer is introduced.

## Ghi chú tiếng Việt

Folder này sở hữu ranh giới `PackageAdapter` của Lib. `package_adapter.rs` chỉ triển khai trait và chuyển tiếp từng trách nhiệm sang owner con tương ứng.

| Owner | Trách nhiệm |
| --- | --- |
| `capability_profile/` | Mảng capability công khai và chọn claim theo ngôn ngữ |
| `manifest_identity/` | Identity ổn định của ngôn ngữ/manifest |
| `age_gate/` | Hook chính sách age-gate theo project |
| `add_preparation/` | Chuẩn bị add qua resolve-first native |
| `manifest_parse/` | Điều phối parse manifest chỉ đọc |
| `installed_inventory/` | Inventory package đã cài có thể xác minh |
| `web_preferences/` | Chuyển tiếp tùy chọn riêng cho Web |

`mod.rs` chỉ nối các owner và re-export helper ở phạm vi nội bộ adapter. Mọi lỗi tiếp tục dùng `MgError`/`MgResult` chung; không tạo lớp lỗi song song. Test hiện hữu được giữ nguyên, chưa chạy hay di chuyển trong phase refactor source.
