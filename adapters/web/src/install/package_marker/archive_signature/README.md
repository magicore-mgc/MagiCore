# Archive signature owner / Owner chữ ký archive

Owns gzip/tar signature orchestration. `entry_scan/` skips directories and GNU `g`/`x` metadata entries, rejects links and unsupported entry types, validates regular-file sizes, and hashes contents. `tree_signature/` strips a common archive root, sorts normalized paths, then hashes the shared tree framing.

Sở hữu điều phối chữ ký gzip/tar. `entry_scan/` bỏ directory và entry metadata GNU `g`/`x`, từ chối link/entry type không hỗ trợ, kiểm tra kích thước file thường và hash nội dung. `tree_signature/` bỏ root chung, sắp xếp path chuẩn hóa rồi hash theo framing cây dùng chung.

Path policy is isolated in `archive_path/`; executable-mode mapping and tree framing are shared package-marker owners. Owner-local tests cover archive orchestration, entry scanning, and empty-tree signing. Errors keep the current `MgError::Other` text.

Policy path nằm riêng trong `archive_path/`; executable mode và tree framing dùng owner marker chung. Test gần owner bao phủ điều phối archive, quét entry và ký cây rỗng. Lỗi giữ nguyên text `MgError::Other` hiện tại.
