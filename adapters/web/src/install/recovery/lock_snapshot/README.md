# Lock snapshot and rollback

## Purpose

Owns rollback-critical lock handling: restoring the prior `mgc.lock` bytes (or removing the lock a failed install wrote), snapshotting prior lock bytes without collapsing I/O errors into "no lock existed", and sweeping stale `.mgc-snap-*` directories without following symlinks.

## Vietnamese

Sở hữu xử lý lock quan trọng cho rollback: khôi phục byte `mgc.lock` cũ (hoặc xoá lock do install lỗi ghi), snapshot byte lock cũ mà không nuốt lỗi I/O thành "không có lock", và dọn thư mục `.mgc-snap-*` cũ mà không đi theo symlink.
