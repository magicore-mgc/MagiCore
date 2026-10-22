# Policy integration-test owners / Owner test integration policy

This folder decomposes existing lockfile policy integration suites by responsibility while preserving each original Cargo target. `v4_policy/` covers trust mode, root references, enforcement, signing and on-disk verification; `v4_sri_digest_shape/` covers SRI validation, migration conversion and native SHA-256 representation. Root targets use `include!`, not child `mod`s, so test names and lexical scope stay stable.

Thư mục chia suite integration theo trách nhiệm nhưng giữ target gốc. Dùng `include!` để tên test và lexical scope không đổi. Các test chưa chạy.
