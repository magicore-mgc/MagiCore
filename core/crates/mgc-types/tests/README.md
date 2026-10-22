# mgc-types integration tests

Cargo-discovered public API tests for shared types and crate-root exports. Focused JSONC and Ecosystem behavior cases live beside their source owners; `jsonc_test.rs` retains one external export contract, while `ecosystem_test.rs` is a public-path smoke. `publish_test.rs` pins both publish type paths; `error_test.rs` pins the shared error/result root and module exports.

Test integration được Cargo tự phát hiện cho API public và export crate-root. Case thuật toán JSONC và hành vi Ecosystem nằm cạnh source owner; `jsonc_test.rs` giữ contract export, còn `ecosystem_test.rs` là smoke public path. `publish_test.rs` ghim hai public path của publish types; `error_test.rs` ghim export root và module của error/result dùng chung.
