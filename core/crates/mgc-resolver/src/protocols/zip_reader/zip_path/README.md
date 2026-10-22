# ZIP path-safety owner

## English

Converts an archive name into a relative `PathBuf`, ignores `CurDir`, and rejects parent/root/prefix components and empty results. Extraction only joins paths after sanitization; containment and filesystem-link checks remain in the extraction/archive owners.

## Tiếng Việt

Chuyển tên entry thành `PathBuf` tương đối, bỏ qua `CurDir`, từ chối parent/root/prefix và kết quả rỗng. Extraction chỉ ghép path sau sanitize; containment và kiểm tra link filesystem tiếp tục ở owner extraction/archive.
