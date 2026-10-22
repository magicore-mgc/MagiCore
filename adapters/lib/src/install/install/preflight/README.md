# Lib native install preflight / Preflight install native Lib

EN: Owns the existing lock checks for non-TypeScript lanes in their original order: graph coverage, Rust Cargo compatibility validation, then hostile/malformed existing-lock rejection. It performs no downloads, CAS writes or lock publication.

VI: Sở hữu các bước kiểm tra lock hiện tại cho lane không phải TypeScript theo đúng thứ tự: graph coverage, xác minh tương thích Cargo của Rust, rồi từ chối lock cũ hỏng/không an toàn. Owner không tải package, ghi CAS hay công bố lock.
