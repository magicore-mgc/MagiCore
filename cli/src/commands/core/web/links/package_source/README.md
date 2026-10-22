# Web package-source lookup

This owner resolves explicit local paths, project `node_modules`, then the ordered MagiCore global roots. Absolute paths use the platform `Path` semantics, including Windows drive paths. Existence checks, canonicalization and diagnostics otherwise retain the prior contract.

# Tìm source package Web

Owner này tìm path cục bộ tường minh, `node_modules` của project, rồi các global root MagiCore theo thứ tự. Absolute path dùng quy tắc `Path` của từng OS, gồm đường dẫn ổ đĩa Windows. Các kiểm tra tồn tại, canonicalize và diagnostic còn lại giữ contract cũ.
