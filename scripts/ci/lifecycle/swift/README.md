# Lifecycle Swift setup / Setup Swift cho lifecycle

`preserve_python_windows.ps1` copies and validates the dedicated Python 3.10 runtime before Swift installation can mutate the setup-python toolcache. `install_windows.ps1` installs Swift through the existing WinGet route, validates toolchain/runtime/SDKROOT and exports paths for the later lifecycle steps. These are lifecycle-specific contracts; the release-binary workflow uses a signed installer path instead.

`preserve_python_windows.ps1` copy và xác minh Python 3.10 riêng trước khi cài Swift có thể sửa toolcache setup-python. `install_windows.ps1` cài Swift bằng WinGet như hiện tại, kiểm tra toolchain/runtime/SDKROOT rồi xuất path cho step lifecycle sau. Đây là contract riêng; release-binary workflow dùng installer có chữ ký.
