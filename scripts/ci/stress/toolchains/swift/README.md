# Swift verification and Windows setup / Xác minh Swift và cài trên Windows

`install_windows.ps1` validates the installed toolchain, runtime DLL and user `SDKROOT`, then publishes the required path/environment values and verifies Swift. The exact Linux signing-key import is shared by stress and lifecycle at `scripts/ci/toolchains/swift/`.

`install_windows.ps1` kiểm tra toolchain, runtime DLL và `SDKROOT` user, ghi path/environment cần thiết rồi xác minh Swift. Thao tác nhập khóa Linux giống hệt được stress và lifecycle dùng chung tại `scripts/ci/toolchains/swift/`.
