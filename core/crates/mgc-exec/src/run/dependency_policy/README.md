# Dependency policy

Owns the shared dependency-resolution boundary and routes each package/compiler family to one language folder. `dependency_policy.rs` normalizes argv and environment; `languages/` holds per-ecosystem rules; `error.rs` carries typed denials while public callers retain their existing reason/result APIs; `test/` covers shared parsing and error behavior.

# Dependency policy

Sở hữu ranh giới resolve dependency dùng chung và chuyển từng họ package/compiler sang folder ngôn ngữ riêng. `dependency_policy.rs` chuẩn hóa argv/environment; `languages/` giữ rule từng ecosystem; `error.rs` mang lỗi từ chối typed trong khi caller public giữ API reason/result cũ; `test/` kiểm tra parse chung và lỗi.
