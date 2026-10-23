# Python manifest compatibility / Tương thích manifest Python

**Purpose / Mục đích:** Keep the public native-project probe and pyproject bridges delegated to mgc-python-service. / Giữ probe dự án Python native và cầu nối pyproject ủy quyền cho mgc-python-service.

**Errors / Lỗi:** Keep existing mgc_types::MgError / MgResult behavior; do not flatten or replace errors here. Giữ nguyên cách trả lỗi mgc_types::MgError / MgResult; không đổi chính sách lỗi.

**Tests / Kiểm thử:** nearest existing test remains adapters/lib/tests/integration/manifest_io/; this source-first slice does not move or run tests. Test gần nhất vẫn ở adapters/lib/tests/integration/manifest_io/; lát source-first chưa di chuyển hoặc chạy test.
