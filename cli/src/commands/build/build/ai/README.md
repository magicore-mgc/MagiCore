# AI build owner / Owner build AI

## Responsibility / Trách nhiệm

Selects the existing Python, Rust/Candle or Go/TensorFlow build lane from safe project manifests, loads optimizer inputs and invokes only the MagiCore-approved native build route. It does not install packages or delegate to another package manager.

Chọn lane build Python, Rust/Candle hoặc Go/TensorFlow từ manifest project an toàn, nạp optimizer input và chỉ gọi route native được MagiCore cho phép. Không cài package hoặc ủy quyền package manager khác.

## Errors and tests / Lỗi và test

The owner-local `error.rs` defines missing AI framework/toolchain and Python backend failures. Python backend errors retain their source while rendering the existing CLI message. Rust/Go command failures continue to propagate from their current execution boundary. Tests remain in the CLI test owner until the repository source pass closes.

`error.rs` cục bộ định nghĩa lỗi thiếu framework/toolchain AI và lỗi Python backend. Lỗi Python giữ source và diagnostic CLI hiện tại. Lỗi lệnh Rust/Go tiếp tục propagate từ execution boundary đang có. Test giữ tại CLI test owner tới khi đóng source pass.
