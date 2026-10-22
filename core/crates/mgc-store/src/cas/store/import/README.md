# CAS import owners

This folder wires four private owners: file imports, byte imports, the hash-checked atomic writer, and tarball-entry imports. Public methods remain inherent methods on cas::store::ContentStore. Cross-owner writer calls are restricted to this import subtree. All owners use the crate's shared StoreError because file-system, hash and durability failures are common CAS errors.

Thư mục này nối bốn owner private: import file, import byte, writer nguyên tử có kiểm hash và import entry tarball. Method công khai vẫn là inherent method trên cas::store::ContentStore. Lời gọi writer xuyên owner chỉ nằm trong subtree import. Các owner dùng StoreError chung của crate vì lỗi filesystem, hash và durability là lỗi CAS dùng chung.

Existing public callers remain unchanged across adapters/lib, adapters/web, adapters/app, mgc-fetcher and CLI. The frozen black-box suites are core/crates/mgc-store/tests/adversarial_integrity.rs and compiled_cache_integrity.rs. Local owner tests add a direct file import, executable-mode address, poisoned hash cache-hit, and tarball-entry path. All tests remain NOT_RUN until global source decomposition closes.
