# PyPI coordinator tests / Test coordinator PyPI

This directory holds separate tests for coordinator construction/version policy, importable-wheel safety, and marker behavior. Each file is registered from `pypi.rs` with `#[path]`, so the test module retains its existing parent visibility while production source contains no test bodies.

Thư mục này chứa test riêng cho constructor/policy version, an toàn wheel importable và hành vi marker. Mỗi file được `pypi.rs` đăng ký bằng `#[path]`, giữ visibility từ module cha mà không để test body trong production source.
