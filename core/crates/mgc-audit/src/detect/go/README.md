# Go manifest detector

## Purpose

Owns the existing go.mod predicate and creates the existing native Go OSV ScanStep. The implementation lives in go.rs; mod.rs only wires and re-exports the stable go_step function.

## Phạm vi

Sở hữu predicate manifest go.mod hiện có và tạo ScanStep native Go OSV không đổi. Implementation nằm trong go.rs; mod.rs chỉ wiring và export function go_step ổn định.

This owner does not add manifest formats or alter scanner behavior.
