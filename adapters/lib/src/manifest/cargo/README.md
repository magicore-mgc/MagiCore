# Cargo manifest bridge / Cầu nối manifest Cargo

**Purpose / Mục đích:** Delegate Cargo read/write to the shared adapter-base owner. / Ủy quyền đọc/ghi Cargo cho owner dùng chung của adapter-base.

**Errors / Lỗi:** Keep existing mgc_types::MgError / MgResult behavior; do not flatten or replace errors here. Giữ nguyên cách trả lỗi mgc_types::MgError / MgResult; không đổi chính sách lỗi.

**Tests / Kiểm thử:** nearest existing test remains adapters/lib/tests/integration/manifest_io/; this source-first slice does not move or run tests. Test gần nhất vẫn ở adapters/lib/tests/integration/manifest_io/; lát source-first chưa di chuyển hoặc chạy test.
