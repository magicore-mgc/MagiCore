# auth_token_write owner / Owner auth_token_write

Validates and stores registry tokens while refusing symlinks and retaining private Unix permissions.
Kiểm tra và lưu token registry, chặn symlink và giữ quyền Unix riêng tư.

The public API and error contract remain on `mgc_config::npmrc::NpmRc`.
Public API và error contract tiếp tục ở `mgc_config::npmrc::NpmRc`.
