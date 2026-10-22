# Trust pending command owner

This owner implements `mgc trust pending`. `pending.rs` coordinates project access and scan inputs; `report/` classifies the results and renders the stable English CLI output. `policy/`, `scan_lock/`, and `package_scan/` own the remaining inputs and scan behavior. `mod.rs` keeps the public `commands::trust::pending::execute` path stable.

`execute` reads the machine-local trust database and the project `[scripts]` table while holding the project write lock, scans installed packages recursively, merges policy through `mgc_config::project::decide_scripts`, and reports denied, approved, and undecided package identities. It does not install, remove, or execute packages.

The command fails closed when it cannot read a policy or complete the package scan. Local typed errors preserve source causes and stable English text. Nearest tests are in each behavior owner's `test/` directory. Runtime tests are pending the repository-wide source pass and the user's storage limit.

# Owner lệnh trust pending

Owner này triển khai `mgc trust pending`. `pending.rs` điều phối truy cập project và input quét; `report/` phân loại kết quả và in output CLI tiếng Anh ổn định. `policy/`, `scan_lock/` và `package_scan/` sở hữu các input và hành vi quét còn lại. `mod.rs` giữ nguyên đường dẫn công khai `commands::trust::pending::execute`.

`execute` đọc trust database cục bộ và bảng `[scripts]` của project trong lúc giữ project write lock, quét đệ quy package đã cài, gộp policy bằng `mgc_config::project::decide_scripts`, rồi báo package bị từ chối, được duyệt và chưa quyết định. Lệnh không cài, gỡ hoặc chạy package.

Lệnh fail-closed nếu không đọc được policy hoặc không quét đầy đủ package. Lỗi typed cục bộ giữ source và thông báo tiếng Anh ổn định. Test gần từng hành vi nằm trong `test/` của owner tương ứng. Runtime test chờ source pass toàn repo và điều kiện dung lượng của user.
