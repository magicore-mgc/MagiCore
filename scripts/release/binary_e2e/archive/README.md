# Release archive operations

`pack_release_archive.sh` consumes the compiled host binary and `HOST_TRIPLE`, writes the release-compatible tar.gz/zip layout, and exports `STAGE_DIR` through `GITHUB_ENV`. `install_clean_prefix.sh` extracts that archive into the isolated `$HOME/.magicore-test` prefix, verifies the installed binary with `--version`, and never depends on the repository binary being on PATH.

`pack_release_archive.sh` nhận binary đã build cùng `HOST_TRIPLE`, tạo layout tar.gz/zip tương thích release và ghi `STAGE_DIR` qua `GITHUB_ENV`. `install_clean_prefix.sh` giải nén vào prefix cô lập `$HOME/.magicore-test`, kiểm tra binary bằng `--version`, không dựa vào binary trong PATH của repository.

Both scripts run under Bash on Ubuntu, macOS and Windows Git Bash. The Windows branch intentionally uses PowerShell for zip extraction/creation because the Git Bash `tar` is not the native Windows zip handler.

Cả hai script chạy bằng Bash trên Ubuntu, macOS và Windows Git Bash. Nhánh Windows cố ý dùng PowerShell để tạo/giải nén zip vì `tar` của Git Bash không phải bộ xử lý zip native của Windows. Xác minh runtime được hoãn lại.
