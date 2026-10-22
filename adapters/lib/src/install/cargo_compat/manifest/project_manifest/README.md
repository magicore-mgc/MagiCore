# Cargo project manifest owner

EN: Reads `Cargo.toml`, validates the supported single-package subset, and extracts the root name/version.

VI: Đọc `Cargo.toml`, kiểm tra tập single-package được hỗ trợ và lấy tên/version của package gốc.

The owner delegates failures to the sibling `error/` taxonomy and does not resolve dependencies or mutate files.

Owner này chuyển lỗi sang taxonomy ở `error/`, không tự resolve dependency và không sửa file.
