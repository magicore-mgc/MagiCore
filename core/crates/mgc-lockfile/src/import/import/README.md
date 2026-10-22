# Import coordinator / Điều phối import

This owner selects a supported legacy lockfile parser, normalizes imported package rows, records the source lockfile name as `registry-import` provenance, and returns the import report. It does not claim a registry URL when the input does not contain one.

Owner này chọn parser lockfile cũ được hỗ trợ, chuẩn hóa package đã nhập, ghi tên lockfile nguồn thành provenance `registry-import`, rồi trả báo cáo import. Nó không tự gán registry URL khi input không cung cấp.
