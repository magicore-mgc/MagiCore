# Audit fix / Sửa dependency theo audit

This owner finds matching manifest dependencies, widens only those ranges, resolves the candidate graph, and writes the manifest/`mgc.lock` only after resolution succeeds. Existing `MgError` and `MgResult` are the error contract; resolver failure must leave project files untouched.

Owner này tìm dependency khớp trong manifest, nới range của đúng dependency đó, resolve graph candidate rồi mới ghi manifest/`mgc.lock`. `MgError` và `MgResult` hiện có là error contract; resolver lỗi phải giữ nguyên file dự án.

Local tests cover empty findings and a nonmatching manifest entry. Adapter-level success/failure E2E-like tests remain in `src/test/audit_tests.rs`.

Test tại chỗ kiểm tra findings rỗng và manifest không có dependency khớp. Test success/failure adapter-level vẫn ở `src/test/audit_tests.rs`.
