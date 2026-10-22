# Installed-binary lifecycle lanes

Each script here is one independent lane executed by the installed release binary: Rust library, AI/Python, Swift app, Flutter app, lockfile import, warm reinstall, or shared-store output label. The workflow owns lane ordering and OS-specific setup; the script owns project creation and its MGC lifecycle assertions.

Mỗi script ở đây là một lane độc lập chạy bằng binary release đã cài: thư viện Rust, AI/Python, app Swift, app Flutter, nhập lockfile, cài lặp hoặc nhãn output shared-store. Workflow quản lý thứ tự lane và setup theo OS; script quản lý tạo dự án và các assertion lifecycle MGC.

All project package operations go through the installed `mgc` binary. System runtimes/toolchains are provisioned separately by the workflow. Lane-specific lockfile and strict audit checks must remain fail-closed. The future nearest tests should verify command order, generated fixture assertions, and failure propagation without replacing the real GitHub E2E lane.

Mọi thao tác package trong dự án đều đi qua binary `mgc` đã cài. Runtime/toolchain hệ thống được workflow cấp riêng. Kiểm tra lockfile và audit strict theo lane phải tiếp tục fail-closed. Test gần script ở pha sau cần xác minh thứ tự lệnh, assertion fixture và truyền lỗi; không thay thế E2E GitHub thật.
