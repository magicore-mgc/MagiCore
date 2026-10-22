# Regular project text reader / Bộ đọc văn bản project an toàn

Owns the public bounded text-reader route. It rejects symlinks and special files, opens without following the final link, checks the opened handle, enforces the 10 MiB limit, and keeps the existing public `mgc_config::project::read_regular_project_text` path.

Quản lý API đọc văn bản project có giới hạn. Owner từ chối symlink/file đặc biệt, mở file không đi theo link cuối, kiểm tra handle đã mở, giới hạn 10 MiB và giữ nguyên đường dẫn public `mgc_config::project::read_regular_project_text`.
