# Game optimizer causal error sources

`source.rs` owns `std::error::Error::source` and the exact cause retained for each variant. It must not suppress or substitute an I/O/`anyhow` cause. Source-chain assertions live under `test/`.

# Source chain lỗi optimizer Game

`source.rs` sở hữu `std::error::Error::source` và nguyên nhân chính xác được giữ theo từng variant. Không được bỏ hoặc thay cause I/O/`anyhow`. Assertion source chain nằm trong `test/`.
