# manifest_io

English: Owns the `manifest_io` responsibility for the CLI model command.
Tiếng Việt: Sở hữu trách nhiệm `manifest_io` của lệnh model CLI.

## Boundary / Ranh giới

- Implementation lives in `manifest_io.rs`; `mod.rs` only declares and re-exports the owner.
- Preserve the existing `mgc model` command contract and its failure behavior.
- Keep this owner focused; shared behavior remains in the narrow owner that defines it.
