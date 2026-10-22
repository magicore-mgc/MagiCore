# Dependency ownership gate

The root keeps the stable `commands::dep_gate::*` API and coordinates one fail-closed decision. `owner_for` is the single policy source used by runtime gates and capability output. Cross-core coordinator tests live in `test/`; each policy, parser, project gate, context, and error owner registers its own cases from its local `test/` folder. Shared fixtures are test-only in `test_support/`. Process-spawn evidence remains in `cli/tests/dep_gate_canary.rs`.

Runtime gates and capability projection must not duplicate the policy table. Unsupported cells stay closed, compatibility execution remains rejected, and manifest, GC, offline, cloud and adapter identity checks keep their established order. The owner map covers all nine core identifiers; it does not claim every language/framework is natively supported.

## Bản tiếng Việt

Root giữ API ổn định `commands::dep_gate::*` và điều phối một quyết định fail-closed. `owner_for` là nguồn policy duy nhất cho gate runtime và capability output. Test dispatch đa core nằm trong `test/`; mỗi owner policy, parser, project gate, context và error tự đăng ký test từ `test/` gần nhất. Fixture dùng chung chỉ bật khi test trong `test_support/`. Bằng chứng process-spawn vẫn nằm ở `cli/tests/dep_gate_canary.rs`.

Gate runtime và capability projection không được sao chép bảng policy. Cell chưa hỗ trợ tiếp tục bị chặn; compat luôn bị từ chối; thứ tự kiểm tra manifest, GC, offline, Cloud và identity adapter được giữ nguyên. Bảng owner có đủ chín core, không đồng nghĩa mọi ngôn ngữ/framework đều được hỗ trợ native.

Hiện đã tách source/test theo owner và rà soát tĩnh. Test, compiler, formatter và runtime chưa chạy; không đánh dấu PASS cho các bước đó.
