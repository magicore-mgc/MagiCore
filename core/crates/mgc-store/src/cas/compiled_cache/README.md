# Compiled module cache / Cache module đã biên dịch

EN: This folder owns the shared local cache for compiled JavaScript modules. Its public facade keeps `mgc_store::cas::{CompiledCache, CompilationKey, CompiledModule, Loader, COMPILED_CACHE_SCHEMA_VERSION}` stable. The cache key includes source bytes, loader semantics, compiler identity/version and canonical options; records are integrity-checked before serving. This is self-validating against corruption and torn writes, not authenticated against a malicious process running as the same user.

VI: Folder này quản lý cache local dùng chung cho module JavaScript đã biên dịch. Facade giữ nguyên public API `mgc_store::cas::{CompiledCache, CompilationKey, CompiledModule, Loader, COMPILED_CACHE_SCHEMA_VERSION}`. Key gồm source, semantics loader, identity/version compiler và options chuẩn hóa; record được kiểm tra integrity trước khi phục vụ. Cache tự kiểm tra lỗi hỏng/ghi dở nhưng không xác thực được process ác ý chạy cùng user.

- `compiled_cache.rs`: cache facade, path, get and put.
- `loader/`, `compilation_key/`, `compiled_module/`: public model owners.
- `record/`, `output_digest/`: on-disk record validation and digest format.
- `publication/`, `temp_guard/`: hard-link race, quarantine and temporary-file cleanup.
- Nearest existing coverage: `core/crates/mgc-store/tests/compiled_cache_integrity.rs`, `adversarial_integrity.rs`, `compile_fail.rs`; CLI route coverage lives under `cli/src/test/`.
- Schema, path, serialized fields, digest bytes, cache decisions and publication order are compatibility contracts.
