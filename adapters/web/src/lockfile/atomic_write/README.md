# Atomic lock writer / Owner ghi lock nguyên tử

EN: Owns same-directory staging, no-follow creation, fsync, atomic replacement and temporary-file cleanup.
VI: Quản lý staging cùng thư mục, tạo no-follow, fsync, thay atomic và dọn file tạm.

Preserve the mutation gate and all existing error text. This source change does not execute filesystem writes.
