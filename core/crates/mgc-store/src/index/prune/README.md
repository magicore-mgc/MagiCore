# CAS pruning / Dọn CAS

Removes only old, unreferenced regular files found below the CAS blob root and only when the file has no external hardlinks. SQLite remains the reference source; an inconsistent empty database fails closed.

Chỉ xóa file thường đã cũ, không còn được tham chiếu dưới CAS blob root và không có hardlink ngoài. SQLite là nguồn tham chiếu; database rỗng nhưng index không rỗng sẽ fail-closed.
