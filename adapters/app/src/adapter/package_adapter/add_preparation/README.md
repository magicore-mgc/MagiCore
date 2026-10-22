# add_preparation

## Responsibility

Keep dependency mutation Flutter-only, resolve the selected version through the native Pub protocol, and hand the resulting lock packages to `PendingInstallState` for the existing install tail.

## Ghi chú tiếng Việt

Folder này giữ add dependency chỉ dành cho Flutter, resolve phiên bản qua Pub protocol native và bàn giao lock packages cho `PendingInstallState` để install tail hiện có xử lý. Các lane khác tiếp tục bị từ chối rõ ràng.
