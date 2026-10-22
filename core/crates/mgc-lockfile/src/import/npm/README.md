# npm lockfile parser

Parses the npm `package-lock.json` packages map, preserves resolved/integrity data and only reconstructs edges to packages found by exact lookup.

Parse map `packages` của npm `package-lock.json`, giữ dữ liệu resolved/integrity và chỉ dựng cạnh khi exact lookup tìm thấy package.

Version numbers remain advisory; the parser gates on shape and reports untested values.

Version chỉ tạo cảnh báo; parser dựa vào cấu trúc và báo khi gặp giá trị chưa kiểm chứng.
