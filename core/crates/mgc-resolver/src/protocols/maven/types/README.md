# types

## Responsibility

POM dependency data types shared by the Maven engine and adapters.

## Ghi chú tiếng Việt

Folder này giữ riêng phần pom dependency data types shared by the maven engine and adapters. để module Maven nhỏ, có chủ sở hữu rõ ràng và dễ mở rộng. Errors use the shared `MgResult`/`MgError` contract; this folder does not invent a parallel error type. Tests for this owner belong beside it under `test/` when the source-first refactor phase is complete.
