# HTML index document

Owns the index route, fallback document generation, project-folder title escaping and HMR script injection. Existing `index.html` is resolved through the source-file boundary before reading, so an external/hidden symlink falls back instead of exposing its target. Missing or unreadable index files keep using the existing fallback contract.

Sở hữu route index, sinh HTML dự phòng, escape title theo tên thư mục project và chèn script HMR. `index.html` có sẵn được resolve qua ranh giới source-file trước khi đọc; symlink ẩn/ra ngoài dùng fallback thay vì lộ target. Khi index thiếu hoặc không đọc được, vẫn theo hợp đồng fallback hiện tại.
