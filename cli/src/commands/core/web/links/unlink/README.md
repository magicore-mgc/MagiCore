# Web package unlink command

This folder owns the `mgc unlink` command. It removes only filesystem symlinks, refuses ordinary package files/directories, and rejects symlinked scoped parents so `unlink` cannot redirect outside the project's scope directory. Windows directory links use the directory-link removal API. Package-name normalization and input validation are separate owners under `links/`.

# Lệnh unlink package Web

Thư mục này sở hữu lệnh `mgc unlink`. Lệnh chỉ gỡ filesystem symlink, từ chối file/thư mục package thường, đồng thời chặn scope parent là symlink để không chuyển thao tác ra ngoài scope directory của project. Symlink thư mục trên Windows dùng API gỡ directory link. Chuẩn hóa tên package và kiểm tra đầu vào thuộc owner riêng dưới `links/`.
