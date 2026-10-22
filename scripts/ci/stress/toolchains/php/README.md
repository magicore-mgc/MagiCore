# macOS PHP runtime setup / Cài PHP trên macOS

`install_macos.sh` installs PHP with Homebrew, exposes its binary directory to later workflow steps and prints the version. Other operating systems continue using the pinned `setup-php` action directly in the workflow.

`install_macos.sh` cài PHP bằng Homebrew, đưa thư mục binary vào PATH của các step sau và in version. OS khác tiếp tục dùng action `setup-php` ghim phiên bản ngay trong workflow.
