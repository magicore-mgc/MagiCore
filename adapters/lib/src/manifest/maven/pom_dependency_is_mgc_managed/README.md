# Maven ownership predicate / Điều kiện ownership Maven

**Purpose / Mục đích:** Recognize only default compile/runtime, non-optional dependencies as MGC-managed. / Chỉ nhận diện dependency compile/runtime mặc định, không optional là do MGC quản lý.

**Errors / Lỗi:** Keep existing mgc_types::MgError / MgResult behavior; do not flatten or replace errors here. Giữ nguyên cách trả lỗi mgc_types::MgError / MgResult; không đổi chính sách lỗi.

**Tests / Kiểm thử:** nearest existing test remains adapters/lib/src/test/manifest_test.rs; this source-first slice does not move or run tests. Test gần nhất vẫn ở adapters/lib/src/test/manifest_test.rs; lát source-first chưa di chuyển hoặc chạy test.
