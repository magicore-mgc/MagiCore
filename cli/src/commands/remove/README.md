# Shared remove entry

This owner is the MCP/programmatic remove entry. The `run`/`run_many` route remains at `crate::commands::remove`. It loads the project identity and delegates to the selected feature-enabled per-core lane; post-remove installation remains disabled for MCP. Per-core removal and firewall policy stay in `commands/core/remove/`. Existing per-core tests remain in that owner; router tests await the source-first test phase.

## Tiếng Việt

Owner này là entry remove programmatic/MCP. Route `run`/`run_many` tiếp tục ở `crate::commands::remove`. Nó nạp project identity rồi gọi lane per-core được bật feature; MCP vẫn tắt install sau remove. Policy firewall và thao tác từng core giữ ở `commands/core/remove/`. Test per-core hiện có giữ tại owner đó; test router chờ phase test sau source-first.
