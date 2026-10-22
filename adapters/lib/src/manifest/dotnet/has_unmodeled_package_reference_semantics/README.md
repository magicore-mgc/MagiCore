# .NET unsupported-semantics guard / Rào semantics .NET chưa hỗ trợ

**Purpose / Mục đích:** Detect conditional, Update/Remove, and qualified PackageReference semantics before mutation. / Phát hiện Condition, Update/Remove và PackageReference có namespace trước khi mutate.

**Errors / Lỗi:** Keep existing mgc_types::MgError / MgResult behavior; do not flatten or replace errors here. Giữ nguyên cách trả lỗi mgc_types::MgError / MgResult; không đổi chính sách lỗi.

**Tests / Kiểm thử:** nearest existing test remains adapters/lib/src/test/manifest_test.rs; this source-first slice does not move or run tests. Test gần nhất vẫn ở adapters/lib/src/test/manifest_test.rs; lát source-first chưa di chuyển hoặc chạy test.
