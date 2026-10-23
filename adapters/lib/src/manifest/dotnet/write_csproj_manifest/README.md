# .NET manifest writer / Writer manifest .NET

**Purpose / Mục đích:** Preserve fail-closed preflight, exact pinning, stale-reference pruning, and atomic publication. / Giữ preflight fail-closed, pin version chính xác, loại reference cũ và publish nguyên tử.

**Errors / Lỗi:** Keep existing mgc_types::MgError / MgResult behavior; do not flatten or replace errors here. Giữ nguyên cách trả lỗi mgc_types::MgError / MgResult; không đổi chính sách lỗi.

**Tests / Kiểm thử:** nearest existing test remains adapters/lib/tests/integration/manifest_io/; this source-first slice does not move or run tests. Test gần nhất vẫn ở adapters/lib/tests/integration/manifest_io/; lát source-first chưa di chuyển hoặc chạy test.
