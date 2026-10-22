# record_paths / path RECORD

Owns POSIX relative-path checks, component-by-component no-follow metadata checks and exact regular-file tree enumeration. Non-UTF-8 installed names fail closed instead of being converted with lossy replacement characters. Parent directories are checked by path and are not pinned by directory handles.

Sở hữu kiểm tra path POSIX tương đối, metadata không theo link từng component và duyệt cây file thường chính xác. Tên cài không phải UTF-8 bị từ chối thay vì đổi qua ký tự thay thế gây nhập nhằng. Thư mục cha được kiểm tra theo path, chưa được ghim bằng directory handle.
