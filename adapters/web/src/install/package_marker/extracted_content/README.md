# Extracted content owner / Owner nội dung đã extract

Walks the extracted root without following symlinks, skips the marker and directories, rejects non-regular entries, streams each file into SHA-256, and compares count, total bytes, executable state, and tree digest against the expected marker.

Walk root đã extract mà không theo symlink, bỏ marker và directory, từ chối entry không phải file thường, hash mỗi file theo luồng rồi so count, tổng bytes, executable state và tree digest với marker mong đợi.

Tests validate a matching file tree, content mutation, and a marker with no full content signature. Filesystem failures keep their existing `MgError::Other` context. A future behavior/security batch should consider resource limits and path-race policy separately.

Test xác minh cây file khớp, nội dung bị sửa và marker thiếu chữ ký nội dung đầy đủ. Lỗi filesystem giữ context `MgError::Other` cũ. Batch behavior/security sau cần xem riêng giới hạn resource và policy race path.
