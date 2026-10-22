# Backup destination validation / Xác thực đích backup

Owns destination entry-type checks, prospective-root resolution, source-overlap rejection, final no-follow destination preparation, empty-directory enforcement and regular-file size inspection. It reuses the shared Store overlap predicate and feature-local `BackupError` contract.

Quản lý kiểm tra loại entry đích, resolve root dự kiến, từ chối overlap với nguồn, chuẩn bị đích no-follow ở bước cuối, yêu cầu thư mục rỗng và đo kích thước file thường. Owner dùng predicate overlap Store cùng hợp đồng `BackupError` cục bộ của feature.
