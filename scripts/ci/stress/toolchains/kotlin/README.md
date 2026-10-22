# Kotlin compiler setup / Cài Kotlin compiler

`install_unix.sh` installs the pinned Kotlin compiler on Linux/macOS; `install_windows.ps1` does the equivalent on Windows. Both verify the downloaded archive SHA-256 before extraction and add the compiler to the workflow PATH. The workflow step conditions select the matching owner.

`install_unix.sh` cài Kotlin compiler ghim phiên bản trên Linux/macOS; `install_windows.ps1` xử lý Windows. Cả hai kiểm tra SHA-256 archive trước khi giải nén và thêm compiler vào PATH của workflow. Điều kiện step chọn đúng owner.
