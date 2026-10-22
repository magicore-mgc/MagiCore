# Windows shell path owner

`mod.rs` only wires and exports the implementation in `windows.rs`. That implementation updates only the current user's PATH through PowerShell and the Windows environment store. The script-contract test supplements, but does not replace, native Windows execution tests.

## Vietnamese

`mod.rs` chỉ wiring và export implementation trong `windows.rs`. Implementation chỉ cập nhật PATH của user hiện tại qua PowerShell và kho môi trường Windows. Test contract script chỉ bổ sung, không thay thế kiểm thử Windows native.
