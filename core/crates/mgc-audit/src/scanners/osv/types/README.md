# OSV shared types

Owns `OsvPin`, the OSV query/detail response DTOs, and the Serde schema consumed by the request owner. Only `OsvPin` is public; response DTOs are crate-private.

## Phạm vi

Folder này sở hữu `OsvPin` cùng DTO query/detail của OSV và schema Serde mà owner truy vấn sử dụng. Chỉ `OsvPin` là public; DTO phản hồi chỉ dùng nội bộ crate.

Errors keep using shared `MgError` / `MgResult`; this folder introduces no independent error taxonomy.
