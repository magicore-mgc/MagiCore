# Install profile tests

These tests exercise the environment-value parser only. They avoid process-global environment mutation, so they remain deterministic and safe to run in parallel.

# Test profiling install

Các test chỉ kiểm tra parser giá trị môi trường. Test không sửa environment toàn process nên có tính xác định và chạy song song an toàn.
