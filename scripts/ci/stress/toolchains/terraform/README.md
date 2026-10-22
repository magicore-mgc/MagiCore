# Terraform runtime setup / Cài Terraform runtime

`install_unix.sh` selects the pinned Linux/macOS archive and its expected SHA-256 by runner architecture. `install_windows.ps1` verifies the Windows archive. Both expose the verified binary to later workflow steps and check its version.

`install_unix.sh` chọn archive Linux/macOS cùng SHA-256 theo kiến trúc runner. `install_windows.ps1` xác minh archive Windows. Cả hai đưa binary đã xác minh vào PATH của step sau và kiểm tra version.
