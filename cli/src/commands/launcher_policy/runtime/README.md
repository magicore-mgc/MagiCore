# Runtime profiles / Profile runtime

Owns the public `Runtime` enum, its stable lowercase display names, and ordered immutable tables of dangerous flags and Deno permissions. The table contents and their order are copied from the pre-refactor implementation. Read-only slices replace temporary `Vec` allocations during argument validation.

This folder does not decide whether a flag is allowed; that belongs to [argument validation](../argument_validation/README.md). Its tests check names and exact profile contents. No performance benchmark has been run.
