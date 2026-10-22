# Scaffold processor tests

## Purpose

Owns the scaffold processor unit-test module. `processor.rs` is loaded as the existing `tests` child of `scaffold::processor`; `mod.rs` includes behavior groups at that same lexical scope so private imports, helper visibility and harness names remain stable. The suite contains 60 tests and two shared helpers.

## Phạm vi

Sở hữu module unit test của scaffold processor. `processor.rs` vẫn được nạp thành module con `tests` hiện có của `scaffold::processor`; `mod.rs` include các nhóm hành vi trong cùng lexical scope để giữ import private, helper và tên harness. Bộ test có 60 test và hai helper dùng chung.

Tests have not been run in this source-first refactor phase.

Test chưa được chạy trong phase refactor source trước tiên.
