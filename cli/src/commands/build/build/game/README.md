# Game build owner / Owner build Game

## Responsibility / Trách nhiệm

Detects the game engine and builds the currently implemented Bevy/Rust lane. Godot, Unity and Unreal remain explicit unsupported build routes; this module must not claim they build successfully.

Nhận diện game engine và build lane Bevy/Rust hiện được triển khai. Godot, Unity và Unreal vẫn là route build chưa hỗ trợ tường minh; module không được tuyên bố chúng build thành công.

## Errors and tests / Lỗi và test

The owner-local `error.rs` owns unsupported Godot/Unity/Unreal build diagnostics and missing-engine detection. It preserves the existing CLI text and does not alter the Bevy build route. Tests remain in the CLI test owner until the repository source pass closes.

`error.rs` cục bộ sở hữu diagnostic build Godot/Unity/Unreal chưa hỗ trợ và lỗi không tìm thấy engine. Diagnostic CLI hiện tại được giữ nguyên, không đổi route build Bevy. Test giữ tại CLI test owner tới khi đóng source pass.
