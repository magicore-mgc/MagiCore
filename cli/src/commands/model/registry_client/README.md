# registry_client

English: Owns the `registry_client` responsibility for the CLI model command.
Tiếng Việt: Sở hữu trách nhiệm `registry_client` của lệnh model CLI.

## Boundary / Ranh giới

- Implementation lives in `registry_client.rs`; `mod.rs` only declares and re-exports the owner.
- Preserve the existing `mgc model` command contract and its failure behavior.
- Keep this owner focused; shared behavior remains in the narrow owner that defines it.
