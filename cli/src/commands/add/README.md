# Shared add entry

This owner is the MCP/programmatic add entry. `run` and `run_many` remain available at `crate::commands::add`; the coordinator rejects global installs, loads project identity, builds an internal request, then calls the compile-time-gated all-core dispatcher. It uses the shared error factories and keeps post-add installation disabled for MCP. Per-core mutation policy remains in `commands/core/add/`. Existing per-core tests remain under that owner; router tests are deferred to the source-first test phase.

## Tiếng Việt

Owner này là entry add programmatic/MCP. `run` và `run_many` vẫn giữ tại `crate::commands::add`; coordinator từ chối global install, nạp project identity, tạo request nội bộ rồi gọi dispatcher all-core theo compile-time feature. Lỗi dùng factory chung và MCP vẫn tắt install sau add. Policy mutation từng core tiếp tục ở `commands/core/add/`. Test lane per-core hiện có vẫn ở owner đó; test router để phase test sau source-first.
