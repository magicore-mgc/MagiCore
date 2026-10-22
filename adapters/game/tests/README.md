# Game adapter integration tests

The integration.rs Cargo target verifies the crate public API, engine detection, manifest parsing and visible fail-closed behavior. It keeps 21 ordered test wrappers and delegates test bodies to focused owners in integration/. Owner-private unit tests remain under src/**/test/.

Cargo target integration.rs kiểm tra API công khai, nhận diện engine, parse manifest và hành vi fail-closed nhìn thấy từ bên ngoài. Target giữ 21 wrapper theo thứ tự cũ và chuyển thân test vào owner nhỏ trong integration/. Unit test nội bộ vẫn ở src/**/test/.
