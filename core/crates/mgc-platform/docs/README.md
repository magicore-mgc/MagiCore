# mgc-platform

<!-- English module guide: purpose, source map, and review boundaries. -->
<!-- Hướng dẫn tiếng Việt về chức năng và ranh giới review của module. -->

## Purpose

Provides current low-level cross-platform filesystem APIs: standard paths, a bounded filesystem-write semaphore, reflink support, and no-follow regular-file opening. Its public module list is narrower than some historical descriptions; do not assume permission APIs without checking source.

## Source map

- [`lib.rs`](../src/lib.rs)
- [`paths/mod.rs`](../src/paths/mod.rs)
- [`paths/shell/`](../src/paths/shell/) — shared PATH API and OS-specific owners.
- [`fs_semaphore.rs`](../src/fs_semaphore/)
- [`reflink/`](../src/reflink/mod.rs) — stable dispatch, per-platform implementation owners and local error owner.
- [`open_regular_file/`](../src/open_regular_file/)

## Handoff and review

Keep fallback behavior explicit when reflink is unavailable. Verify path behavior per OS and ensure new writes respect the shared concurrency limit.

## Verification

After code changes, run this crate suite from the repository root with `cargo test -p mgc-platform`. For this source-first task, runtime verification is intentionally deferred until the whole-repository source pass closes.
