# Web dependency update

## Purpose

Owns `mgc update` behavior for core-web. `version_preference/` picks the latest eligible version and preserves the caller's range style; `run/` walks all four dependency tables, enforces the age gate (skipping too-young targets with a loud warning), and rewrites the manifest only when something changed. Public routes `crate::update::{run_update, preferred_registry_version, preferred_saved_range}` are unchanged.

## Phạm vi

Owner này sở hữu hành vi `mgc update` cho core-web. `version_preference/` chọn version hợp lệ mới nhất và giữ kiểu range của người dùng; `run/` duyệt cả bốn bảng dependency, áp age gate (bỏ qua gói quá trẻ kèm cảnh báo rõ) và chỉ ghi manifest khi có thay đổi. Route public `crate::update::{run_update, preferred_registry_version, preferred_saved_range}` không đổi.
