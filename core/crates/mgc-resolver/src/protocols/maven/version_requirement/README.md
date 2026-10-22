# version_requirement

## Responsibility

Maven range matching and version comparison rules.

## Ghi chú tiếng Việt

Folder này giữ riêng phần maven range matching and version comparison rules. để module Maven nhỏ, có chủ sở hữu rõ ràng và dễ mở rộng. Errors use the shared `MgResult`/`MgError` contract; this folder does not invent a parallel error type. Tests for this owner belong beside it under `test/` when the source-first refactor phase is complete.
