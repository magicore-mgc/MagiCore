# Rust Cargo manifest parser / Bộ phân tích Cargo.toml của Rust

Parses the existing `[package]` name and root `[dependencies]` mapping only. Path dependency tables remain `workspace:*`; versioned tables use their version or `*`. Invalid/unrecognized TOML continues to return `None` so the reader can fall through to Python.
