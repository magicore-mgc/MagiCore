# package_adapter

## Responsibility

`package_adapter.rs` keeps the `PackageAdapter for AppAdapter` implementation and delegates each concern to a focused owner.

| Owner | Responsibility |
| --- | --- |
| `flutter_sdk_dependencies/` | Flutter implicit SDK dependencies and lock-replay closure |
| `manifest_identity/` | Stable App language/manifest identity |
| `capability_profile/` | Per-language capability selection |
| `manifest_parse/` | Dispatch to the App manifest parser |
| `native_update_policy/` | Whether native update is supported |
| `add_preparation/` | Flutter-only native resolve-first add and lock handoff |
| `installed_inventory/` | Report only verifiable installed package state |

The public capability arrays remain on `AppAdapter`; pending lock/protocol state remains owned by `PendingInstallState`. Errors continue through `MgError`/`MgResult` without a duplicate local error layer.

## Ghi chú tiếng Việt

`package_adapter.rs` giữ implementation `PackageAdapter for AppAdapter` và chuyển từng trách nhiệm sang owner con tương ứng.

| Owner | Trách nhiệm |
| --- | --- |
| `flutter_sdk_dependencies/` | Dependency SDK Flutter ngầm và closure cho lock replay |
| `manifest_identity/` | Identity ổn định của ngôn ngữ/manifest App |
| `capability_profile/` | Chọn capability theo từng ngôn ngữ |
| `manifest_parse/` | Điều phối sang parser manifest App |
| `native_update_policy/` | Quyết định lane có hỗ trợ native update |
| `add_preparation/` | Resolve-first add chỉ cho Flutter và bàn giao lock |
| `installed_inventory/` | Chỉ báo trạng thái package đã cài có thể xác minh |

Các mảng capability công khai vẫn ở `AppAdapter`; trạng thái lock/protocol chờ install vẫn do `PendingInstallState` sở hữu. Lỗi tiếp tục đi qua `MgError`/`MgResult`, không tạo lớp lỗi song song.
