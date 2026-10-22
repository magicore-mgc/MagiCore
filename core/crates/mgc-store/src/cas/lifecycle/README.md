# CAS lifecycle / Lifecycle CAS

This facade preserves `cas::lifecycle::*` while delegating root identity/symlink validation, ancestor traversal, directory creation, and permission tightening to separate owners. Shared failures use `StoreError`; no local error taxonomy is needed.

Facade này giữ nguyên `cas::lifecycle::*` và chuyển validate root/symlink, duyệt ancestor, tạo thư mục và siết permission cho các owner riêng. Lỗi dùng chung `StoreError`; không cần error taxonomy cục bộ.
