# Installed package scan owner

`package_scan.rs` walks top-level, scoped, and nested `node_modules` directories with a canonical-path visited set. It delegates manifest interpretation to `package_json/` and bounded no-follow file access to `bounded_manifest/`.

The scan returns each scripted package's manifest name/version pair. Incomplete directory reads remain errors so the report cannot silently hide a lifecycle hook.

# Owner quét package đã cài

`package_scan.rs` duyệt `node_modules` cấp đầu, package scoped và cây lồng nhau bằng visited set theo canonical path. Owner chuyển việc hiểu manifest cho `package_json/` và việc đọc file giới hạn/no-follow cho `bounded_manifest/`.

Kết quả quét là các cặp tên/phiên bản lấy từ manifest package có script. Lỗi đọc dở cây thư mục vẫn là lỗi để báo cáo không âm thầm bỏ qua lifecycle hook.
