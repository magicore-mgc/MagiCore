# Go module parser / Parser Go module

**Purpose / Mục đích:** Build the effective Go require graph after project replace and exclude directives. / Dựng graph require Go hiệu lực sau directive replace và exclude của dự án.

**Errors / Lỗi:** Keep existing mgc_types::MgError / MgResult behavior; do not flatten or replace errors here. Giữ nguyên cách trả lỗi mgc_types::MgError / MgResult; không đổi chính sách lỗi.

**Tests / Kiểm thử:** nearest existing test remains adapters/lib/src/test/manifest_test.rs; this source-first slice does not move or run tests. Test gần nhất vẫn ở adapters/lib/src/test/manifest_test.rs; lát source-first chưa di chuyển hoặc chạy test.
