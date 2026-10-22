# Optimizer CLI entry

## Purpose

Owns the `mgc optimizer` command entry: resolves `ProjectContext` (explicit `--core` or project marker), maps the ecosystem to a core name, then delegates to `optimize_project`. `mod.rs` only wires and exports `run/run.rs`.

## Phạm vi

Owner này sở hữu điểm vào lệnh `mgc optimizer`: resolve `ProjectContext` (theo `--core` hoặc marker project), map ecosystem sang tên core rồi chuyển tiếp cho `optimize_project`. `mod.rs` chỉ wiring và export `run/run.rs`.
