# Launcher policy / Policy launcher

This owner preserves the public `commands::launcher_policy::{Runtime, LauncherPolicy}` API used by the shared CLI test runner and Web dev launcher. It coordinates three responsibilities: runtime flag profiles, ordered argument validation, and typed local policy errors.

Current source references show direct callers in `cli/src/commands/test/test.rs` and `cli/src/commands/core/web/dev/runtime_policy/runtime_policy.rs`. The former header's AI/App/Lib claim is not backed by direct source calls in this checkout; those cores can use the shared API later, but are not reported as current consumers.

The policy blocks the same Bun, Deno, and Node flags and Deno permissions using the same `starts_with` checks and check order. Static profile slices avoid rebuilding heap vectors for each argument. This is a source-level allocation reduction only; runtime performance has not been benchmarked.

Tests are registered under each behavior owner's nearest `test/` directory. The compatibility facade tests cover constructors. See [runtime](runtime/README.md), [argument validation](argument_validation/README.md), and [errors](error/README.md).

## Safety and limits

The validation scope and policy tables are unchanged. `allow_dangerous_permissions` only gates Deno permission arguments; always-blocked flags remain blocked. This module does not resolve packages or launch processes. GitNexus returned `UNKNOWN` for exact targets, so graph risk remains unresolved. No test, compiler, CLI, CI, or OS result is implied by source layout.
