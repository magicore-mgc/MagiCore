# Add command schema

This owner defines the Clap variants and arguments for the per-core `add-*`
commands. `mod.rs` only exports `AddCommands`; dispatch and package lifecycle
behavior belong to their respective owners. Clap owns parser diagnostics here,
so this schema has no local error type.

## Tiếng Việt

Folder này khai báo các variant và tham số Clap cho lệnh `add-*` theo core.
`mod.rs` chỉ export `AddCommands`; dispatch và vòng đời package nằm ở owner
tương ứng. Clap chịu trách nhiệm lỗi parse nên schema này không cần error type
riêng.
