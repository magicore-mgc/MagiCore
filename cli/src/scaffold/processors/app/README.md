# App scaffold owners

## Responsibility
This folder owns the App fallback generator reached by the core scaffold writer. It preserves the AppProcessor API and routes single-framework and multi-platform file generation to focused owners.

## Phạm vi
Folder này quản lý generator fallback App được core scaffold writer gọi. Folder giữ API AppProcessor và route việc sinh file framework đơn hoặc đa nền tảng tới owner tương ứng.

## Boundaries
Template selection, metadata persistence, dependency installation and lifecycle commands belong to other owners. Existing callers and tests stay in their current locations.

## Ranh giới
Chọn template, lưu metadata, cài dependency và lệnh lifecycle thuộc owner khác. Caller và test hiện có giữ nguyên vị trí.
