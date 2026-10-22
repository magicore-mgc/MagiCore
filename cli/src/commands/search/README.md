# Search command owners / Owner của lệnh tìm kiếm

This folder owns the shared `mgc search` command coordinator and its focused request, mapping, classification, output, type and error modules. `mod.rs` preserves the public `commands::search::{run, run_with_core}` paths. The command continues to use the existing Web registry endpoint; this structure does not claim all-core search support.

Thư mục này quản lý coordinator của `mgc search` dùng chung cùng các module request, ánh xạ kết quả, phân loại, output, type và lỗi. `mod.rs` giữ nguyên đường dẫn công khai `commands::search::{run, run_with_core}`. Lệnh vẫn dùng endpoint Web registry hiện tại; cấu trúc này không khẳng định đã hỗ trợ tìm kiếm mọi core.
