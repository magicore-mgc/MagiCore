# Maven dependency pruning / Loại dependency Maven cũ

**Purpose / Mục đích:** Remove only MGC-managed top-level pins missing from the resolved manifest. / Chỉ xóa pin top-level do MGC quản lý nhưng không còn trong manifest đã resolve.

**Errors / Lỗi:** Keep existing mgc_types::MgError / MgResult behavior; do not flatten or replace errors here. Giữ nguyên cách trả lỗi mgc_types::MgError / MgResult; không đổi chính sách lỗi.

**Tests / Kiểm thử:** nearest existing test remains adapters/lib/src/test/manifest_test.rs; this source-first slice does not move or run tests. Test gần nhất vẫn ở adapters/lib/src/test/manifest_test.rs; lát source-first chưa di chuyển hoặc chạy test.
