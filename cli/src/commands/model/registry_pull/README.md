# registry_pull

English: Owns the `registry_pull` responsibility for the CLI model command.
Tiếng Việt: Sở hữu trách nhiệm `registry_pull` của lệnh model CLI.

## Boundary / Ranh giới

- Implementation lives in `registry_pull.rs`; `mod.rs` only declares and re-exports the owner.
- Preserve the existing `mgc model` command contract and its failure behavior.
- Keep this owner focused; shared behavior remains in the narrow owner that defines it.
