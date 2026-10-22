# Compiled-cache integrity test suite

This owner contains the implementation modules for the stable Cargo integration target `test_compiled_cache_integrity`. The root target keeps all original test names, registration order, attributes and cfg gates as wrappers; behavior implementations are grouped under `test/`, while fixtures stay local under `support/`.

Owner này chứa module implementation cho Cargo integration target ổn định `test_compiled_cache_integrity`. File target gốc giữ nguyên tên test, thứ tự đăng ký, attribute và cfg bằng wrapper; implementation được nhóm theo hành vi trong `test/`, fixture nằm cục bộ trong `support/`.

No test was executed during this source-first refactor. The generation-claim test is named `staging_claims_survive_other_promote_and_abort_is_scoped`; it exercises staging-claim retention and scoped abort behavior, not cache prune or an unavailable database.

Batch này chưa chạy test. Test generation claim có tên `staging_claims_survive_other_promote_and_abort_is_scoped`; test kiểm tra claim staging được giữ lại và abort chỉ ảnh hưởng generation của nó, không kiểm tra cache prune hay database không khả dụng.
