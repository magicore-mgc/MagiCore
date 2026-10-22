# Rust library fallback scaffold

## Responsibility
Creates a standalone Cargo package, deterministic minimal lockfile, and a real unit assertion. The current wildcard routes any unrecognized framework here; that behavior is preserved for a separate fail-closed change.

## Phạm vi
Tạo Cargo package độc lập, lockfile tối thiểu tất định và unit assertion thật. Wildcard hiện tại đưa mọi framework không nhận diện vào đây; behavior được giữ để sửa fail-closed ở task riêng.

## Tests / Kiểm thử
The nearest source tests are in `test/`; they are source-only until repository-wide refactoring closes.

Test gần owner nằm trong `test/`; hiện chỉ là source cho tới khi refactor toàn repo hoàn tất.
