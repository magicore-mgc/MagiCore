# Executable metadata owner / Owner metadata executable

Owns the projection from tar header mode and filesystem metadata to the package-tree executable bit. Unix checks any `0o111` bit; non-Unix retains the previous conservative `false` behavior.

Sở hữu phép chiếu mode của tar header và metadata filesystem thành cờ executable của cây package. Unix kiểm tra mọi bit `0o111`; non-Unix giữ hành vi bảo thủ `false`.

Tests are cfg-specific under `test/executable.rs`. This owner is infallible and needs no local error/type module.

Test theo cfg ở `test/executable.rs`. Owner không phát sinh lỗi và không cần module error/type cục bộ.
