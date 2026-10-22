# pom_coordinates

## Responsibility

Parent/project coordinate parsing and property substitution.

## Ghi chú tiếng Việt

Folder này giữ riêng phần parent/project coordinate parsing and property substitution. để module Maven nhỏ, có chủ sở hữu rõ ràng và dễ mở rộng. Errors use the shared `MgResult`/`MgError` contract; this folder does not invent a parallel error type. Tests for this owner belong beside it under `test/` when the source-first refactor phase is complete.
