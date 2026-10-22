# ESP32 Rust Cargo package names

This owner validates the stable Cargo manifest package-name form before ESP32 scaffolding mutates the filesystem. It accepts a Unicode identifier start or `_`, followed by Unicode identifier continuation characters or `-`. It does not implement `cargo new`'s additional project naming rules or validate that generated firmware builds.

Owner này xác thực dạng tên package trong manifest Cargo stable trước khi scaffold ESP32 thay đổi filesystem. Tên bắt đầu bằng Unicode identifier hoặc `_`, sau đó dùng Unicode identifier hoặc `-`. Owner không triển khai các rule bổ sung của `cargo new` và không xác nhận firmware sinh ra build được.
