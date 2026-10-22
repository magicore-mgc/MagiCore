# Release E2E signing and toolchain setup

`import_swift_signing_keys.sh` downloads Swift.org's published key bundle and checks both required fingerprints before signature verification can proceed. `install_swift_windows.ps1` downloads the pinned Swift installer, checks the SHA-256 and Authenticode signature, installs it, verifies compiler/runtime files, and exports `SDKROOT` and runtime paths for later workflow steps.

`import_swift_signing_keys.sh` tải key bundle do Swift.org công bố và xác minh đủ hai fingerprint trước khi tiếp tục kiểm tra chữ ký. `install_swift_windows.ps1` tải installer Swift đã ghim, kiểm tra SHA-256 và Authenticode, cài đặt, xác minh compiler/runtime rồi xuất `SDKROOT` và runtime path cho các step sau.

The workflow applies OS conditions and pins the Python/Swift versions. These scripts do not authorize floating versions or unsigned installers. Add a focused script contract test in the later test phase; GitHub runner behavior remains unverified here.

Workflow áp dụng điều kiện OS và ghim phiên bản Python/Swift. Các script không cho phép phiên bản trôi nổi hoặc installer không chữ ký. Pha test sau cần thêm contract test riêng; hành vi trên GitHub runner hiện chưa được xác minh.
