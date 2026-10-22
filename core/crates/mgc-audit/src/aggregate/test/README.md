# Aggregate tests

This folder owns the registered aggregate_test integration target for aggregate_reports. The source is compiled as an integration crate through the explicit Cargo target and imports the public mgc_audit API. Keeping it here makes the test owner visible beside the behavior without placing test bodies inside production modules.

The source relocation preserves all 9 existing test names and 21 assertion macros. These tests remain NOT_RUN until the source-first refactor closes and the workspace storage gate is met.

## Bản tiếng Việt

Folder này sở hữu integration target aggregate_test đã đăng ký cho aggregate_reports. Source được Cargo biên dịch như integration crate và gọi API công khai mgc_audit. Test nằm cạnh behavior nhưng body không đặt trong module production.

Việc chuyển file giữ nguyên 9 tên test và 21 assertion macro. Các test vẫn NOT_RUN cho đến khi giai đoạn refactor source kết thúc và đạt gate dung lượng.
