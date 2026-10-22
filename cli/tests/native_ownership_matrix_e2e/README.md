# Native ownership matrix E2E / E2E ma trận native ownership

This folder decomposes the 3,001-line Cargo integration test into behavior owners. The Cargo target remains `cli/tests/native_ownership_matrix_e2e.rs`; that root file keeps one test wrapper per original name because CI uses exact-name filters. `test/` contains assertion bodies, `support/` contains reusable product-test fixtures and bounded process helpers, and `mod.rs` files only wire and export owners.

Thư mục này tách integration test Cargo 3.001 dòng thành owner theo hành vi. Cargo target vẫn là `cli/tests/native_ownership_matrix_e2e.rs`; file gốc giữ wrapper cho từng tên test vì CI lọc tên chính xác. `test/` chứa thân assertion, `support/` chỉ chứa fixture tái dùng và helper process có giới hạn; `mod.rs` chỉ nối và export owner.

Không có `error/` vì lát cắt này không tạo kiểu lỗi nghiệp vụ mới. Không có luồng cài package mới trong thư mục này.
