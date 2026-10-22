# Native Cloud deployment

The API currently fails closed because MagiCore has no native provider plan/apply engine. It never starts a provider CLI or fabricates dry-run output.

API hiện fail-closed vì MagiCore chưa có provider plan/apply engine native. API không chạy CLI provider và không giả kết quả dry-run.

The existing unit test remains registered at `test/mod_test.rs`.

Unit test hiện có tiếp tục đăng ký tại `test/mod_test.rs`.
