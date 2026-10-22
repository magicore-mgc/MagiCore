# CAS import publication locking

A bounded process-wide set of digest stripes serializes cache lookup through publication for duplicate byte/stream imports across independent ContentStore handles. This prevents a redundant rename from changing an inode while a sibling verifies it. Unrelated digests can proceed concurrently, with bounded stripe collisions. No filesystem lock files or unbounded digest map are allocated.

Khóa stripe theo digest dùng chung trong process giữ lookup → publish nhất quán giữa các handle, tránh rename trùng làm thay inode đang xác minh. Không bỏ kiểm tra hash/ctime và không thêm lockfile hoặc map tăng vô hạn.

This coordinates one process only. Concurrent independent processes still retain fail-closed verification errors; no cross-process serialization is claimed. Regression: ../write_bytes/test/write_bytes.rs tests actual synchronized duplicate imports and final bytes.
