# .NET reference pruning / Loại PackageReference .NET

**Purpose / Mục đích:** Remove references absent from the resolved keep set while preserving other bytes. / Xóa reference ngoài keep-set đã resolve và giữ nguyên các byte khác.

**Errors / Lỗi:** Keep existing mgc_types::MgError / MgResult behavior; do not flatten or replace errors here. Giữ nguyên cách trả lỗi mgc_types::MgError / MgResult; không đổi chính sách lỗi.

**Tests / Kiểm thử:** nearest existing test remains adapters/lib/src/test/manifest_test.rs; this source-first slice does not move or run tests. Test gần nhất vẫn ở adapters/lib/src/test/manifest_test.rs; lát source-first chưa di chuyển hoặc chạy test.
