# IoT build owner / Owner build IoT

## Responsibility / Trách nhiệm

Accepts the existing MagiCore Rust build path, including managed Cargo source validation. PlatformIO and Zephyr manifests fail closed because their external build commands may resolve or install dependencies outside MagiCore.

Nhận route build Rust hiện có của MagiCore, bao gồm kiểm tra nguồn Cargo được quản lý. Manifest PlatformIO và Zephyr bị từ chối fail-closed vì lệnh build ngoài có thể tự resolve/cài dependency ngoài MGC.

## Errors and tests / Lỗi và test

The owner-local `error.rs` owns PlatformIO/Zephyr fail-closed build refusals, missing Cargo and missing IoT framework errors. The provider commands remain uninvoked and their dependency-resolution refusal messages stay unchanged. Tests remain in the CLI test owner until the repository source pass closes.

`error.rs` cục bộ sở hữu lỗi từ chối build fail-closed PlatformIO/Zephyr, thiếu Cargo và không tìm thấy framework IoT. Lệnh provider vẫn không được gọi và diagnostic từ chối resolve/cài dependency giữ nguyên. Test giữ tại CLI test owner tới khi đóng source pass.
