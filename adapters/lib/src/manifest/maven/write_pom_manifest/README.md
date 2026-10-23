# Maven manifest writer / Writer manifest Maven

**Purpose / Mục đích:** Validate exact pins, upsert managed dependencies, prune removed pins, then atomically publish pom.xml. / Kiểm tra pin chính xác, upsert dependency được quản lý, xóa pin đã bỏ rồi publish pom.xml nguyên tử.

**Errors / Lỗi:** Keep existing mgc_types::MgError / MgResult behavior; do not flatten or replace errors here. Giữ nguyên cách trả lỗi mgc_types::MgError / MgResult; không đổi chính sách lỗi.

**Tests / Kiểm thử:** nearest existing test remains adapters/lib/tests/integration/manifest_io/; this source-first slice does not move or run tests. Test gần nhất vẫn ở adapters/lib/tests/integration/manifest_io/; lát source-first chưa di chuyển hoặc chạy test.
