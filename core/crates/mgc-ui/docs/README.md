# mgc-ui

<!-- English module guide: purpose, source map, and review boundaries. -->
<!-- Hướng dẫn tiếng Việt về chức năng và ranh giới review của module. -->

## Purpose

Provides terminal presentation primitives: help, progress bars/spinners, prompts, tables, status output, and quiet mode. It owns reusable presentation, not command policy or business decisions.

## Source map

- [source facade](../src/README.md)
- [crate exports](../src/lib.rs)
- [banner owner](../src/banner/README.md)
- [command-style owner](../src/command_style/README.md)
- [help owner](../src/help/README.md)
- [install-summary owner](../src/install_summary/README.md)
- [next-steps owner](../src/next_steps/README.md)
- [progress owner](../src/progress/README.md)
- [quiet-state owner](../src/quiet/README.md)
- [section owner](../src/section/README.md)
- [status-message owner](../src/status/README.md)
- [prompt owner](../src/prompt/README.md)
- [table owner](../src/table/README.md)

## Handoff and review

Keep terminal output English per RULE.md; preserve quiet and non-TTY behavior. Presentation text does not validate command capability or replace engine diagnostics.

## Verification

The source-first refactor has authored tests that remain NOT_RUN. Once whole-source closure is reached and total machine-used storage is below 80 GB, run the nearest crate tests and cargo test -p mgc-ui.
