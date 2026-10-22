# archive_index / index archive

Owns duplicate-safe indexing of authenticated ZIP member names and selection of the single root `dist-info/RECORD` and `METADATA`. It stores entry positions, so the decompressed payload vectors are not cloned.

Folder này sở hữu index tên member ZIP đã xác thực, chặn tên trùng và chọn `dist-info/RECORD`/`METADATA` duy nhất ở root. Index lưu vị trí entry, không clone vector payload đã giải nén.
