# Rust offline install / Install Rust offline

This owner preflights every locked Cargo archive without writes, then reopens and verifies each archive before CAS import and Cargo-compatible materialization. It preserves the error when a preflighted cache entry disappears.

Owner này preflight mọi archive Cargo trong lock mà không ghi dữ liệu, sau đó đọc và xác minh lại từng archive trước khi import CAS và materialize layout Cargo tương thích. Owner giữ lỗi riêng khi entry biến mất sau preflight.
