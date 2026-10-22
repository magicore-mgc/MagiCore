# pom_dependencies

## Responsibility

Top-level project dependency extraction and scope filtering.

## Ghi chú tiếng Việt

Folder này giữ riêng phần top-level project dependency extraction and scope filtering. để module Maven nhỏ, có chủ sở hữu rõ ràng và dễ mở rộng. Errors use the shared `MgResult`/`MgError` contract; this folder does not invent a parallel error type. Tests for this owner belong beside it under `test/` when the source-first refactor phase is complete.
