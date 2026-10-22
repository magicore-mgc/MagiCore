# Resolved manifest owner / Owner resolved manifest

This owner implements root-package membership and projects registry-selected dependency specs into a cloned `Manifest`. It preserves dependency-group order and the existing fail-fast diagnostics when the resolved list does not match the manifest.

Owner này nhận diện package gốc và đưa resolved spec từ registry vào bản sao `Manifest`. Owner giữ thứ tự dependency group và diagnostic hiện có khi số lượng/tên package không khớp.

Tests live under `test/` and cover root matching, ordered projection, and mismatch rejection. The Web integration tests remain at their registered adapter paths.

Test nằm trong `test/`, kiểm tra root matching, projection theo thứ tự và từ chối mismatch. Web integration test vẫn ở path adapter đã đăng ký.
