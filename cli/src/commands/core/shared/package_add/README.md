# Shared package add

This owner preserves the cross-core `add` route. `package_add.rs` coordinates validation, ordered native resolution, journal-aware manifest commit and post-add install/recovery. Each phase keeps the existing shared errors and side-effect order; no adapter or core contract is redefined here.

## Tiếng Việt

Owner này giữ route `add` dùng chung giữa các core. `package_add.rs` điều phối kiểm tra request, resolve native theo thứ tự, commit manifest có journal và install/recovery sau add. Mỗi phase giữ lỗi dùng chung và thứ tự side effect; không định nghĩa lại contract adapter hay core tại đây.
