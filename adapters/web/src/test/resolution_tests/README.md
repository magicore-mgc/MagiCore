# Web resolution tests

This owner registers package-resolution tests under the existing `unit_tests::resolution_tests` module. It preserves the same library unit-test target and shared fixtures imported from `unit_tests`.

Owner này đăng ký test phân giải package dưới module `unit_tests::resolution_tests` hiện có. Các test vẫn chạy trong cùng library unit-test target và dùng fixture chung từ `unit_tests`.

Each child groups assertions by one resolution responsibility. `mod.rs` files only wire child modules; test cases stay in same-name implementation files.
