# Maven dependency upsert / Cập nhật dependency Maven

**Purpose / Mục đích:** Insert or update a managed top-level Maven dependency while preserving unrelated XML metadata. / Thêm hoặc cập nhật dependency Maven top-level do MGC quản lý và giữ metadata XML không liên quan.

**Errors / Lỗi:** Keep existing mgc_types::MgError / MgResult behavior; do not flatten or replace errors here. Giữ nguyên cách trả lỗi mgc_types::MgError / MgResult; không đổi chính sách lỗi.

**Tests / Kiểm thử:** nearest existing test remains adapters/lib/src/test/manifest_test.rs; this source-first slice does not move or run tests. Test gần nhất vẫn ở adapters/lib/src/test/manifest_test.rs; lát source-first chưa di chuyển hoặc chạy test.
