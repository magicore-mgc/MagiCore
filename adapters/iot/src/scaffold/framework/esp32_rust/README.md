# ESP32 Rust scaffold output

Owns the existing Cargo manifest, MagiCore manifest, `.cargo/config.toml` build target, and `src/main.rs` starter output for ESP32 Rust boards. It validates the Cargo package name before filesystem mutation and writes files only; it does not run Cargo or install crates. The chip-to-target mapping lives in `chip_target/` and fails closed for a chip with no verified triple.

Sở hữu output Cargo manifest, MagiCore manifest, target build trong `.cargo/config.toml` và `src/main.rs` cho board ESP32 Rust. Xác thực tên Cargo trước khi ghi filesystem; không chạy Cargo hoặc cài crate. Mapping chip sang target nằm ở `chip_target/` và fail closed với chip chưa có triple đã xác minh.
