# Workspace discovery / Dò workspace

`workspace.rs` parses `magicore.workspace.toml`, resolves the configured app/package directories through `mgc-workspace` confinement and recursively finds projects without traversing symlink/non-directory entries. Invalid project identity is propagated. Tests live under [`test`](test/README.md).

`workspace.rs` đọc cấu hình, giới hạn đường dẫn qua `mgc-workspace` và tìm project đệ quy; identity lỗi được truyền lên, không bị nuốt. Test nằm trong `test/`.
