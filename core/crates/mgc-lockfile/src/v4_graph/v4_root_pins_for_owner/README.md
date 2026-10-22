# v4_root_pins_for_owner

## Responsibility

Selects the root pins owned by one core, retaining the legacy global-root fallback only when ownership data is absent.

## Trách nhiệm

Chọn root pin của một core; chỉ dùng root toàn cục cho lock cũ không có bảng ownership.

## Contract

This folder owns only this named operation and its module boundary. Existing crate API and test locations stay stable.

Folder này chỉ sở hữu operation có tên và ranh giới module tương ứng. API crate và vị trí test hiện có được giữ nguyên.
