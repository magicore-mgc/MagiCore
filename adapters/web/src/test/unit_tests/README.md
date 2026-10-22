# Web adapter unit test owner

`mod.rs` registers `unit_tests.rs` as a child test module. The implementation is included into the original crate test module, preserving `super::*` access to the same adapter internals. Focused audit/cache/manifest/policy/resolution subjects stay in adjacent `../*_tests.rs` files; install lifecycle cases are split by owner under `../install/`; warm-cache tests stay in their install extraction owner.

`mod.rs` đăng ký `unit_tests.rs` làm module test con. Implementation import crate root tường minh để khi chuyển folder vẫn truy cập đúng internals adapter. Test audit/cache/manifest/policy/resolution theo subject ở `../*_tests.rs`; test lifecycle install tách theo owner dưới `../install/`; test warm-cache nằm cạnh extraction owner.
