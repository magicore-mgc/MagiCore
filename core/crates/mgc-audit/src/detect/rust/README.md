# Rust manifest detector

## Purpose

Owns the existing Cargo.toml predicate and creates the existing native Rust OSV ScanStep. The implementation lives in rust.rs; mod.rs only wires and re-exports the stable rust_step function.

## Phạm vi

Sở hữu predicate manifest Cargo.toml hiện có và tạo ScanStep native Rust OSV không đổi. Implementation nằm trong rust.rs; mod.rs chỉ wiring và export function rust_step ổn định.

This owner does not add manifest formats or alter scanner behavior.
