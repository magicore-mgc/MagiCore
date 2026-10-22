# Game optimizer error variants and constructors

`error.rs` owns the 20 `GameOptimizerError` variants and their constructors, including path capture and original-error formatting. It does not render diagnostics or select source-chain causes; those stay in sibling owners. Constructor-data contracts are tested under `test/`.

# Variant và constructor lỗi optimizer Game

`error.rs` sở hữu 20 variant `GameOptimizerError` và constructor, gồm lưu path và format lỗi ban đầu. File không render diagnostic và không chọn source chain; hai phần đó thuộc owner kế bên. Contract dữ liệu constructor được test trong `test/`.
