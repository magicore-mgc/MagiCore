# Python manifest detector

## Purpose

Owns the existing requirements/pylock/uv.lock/pyproject.toml predicate and creates the existing native Python OSV ScanStep. The implementation lives in python.rs; mod.rs only wires and re-exports the stable python_step function.

## Phạm vi

Sở hữu predicate manifest requirements/pylock/uv.lock/pyproject.toml hiện có và tạo ScanStep native Python OSV không đổi. Implementation nằm trong python.rs; mod.rs chỉ wiring và export function python_step ổn định.

This owner does not add manifest formats or alter scanner behavior.
