# Checked byte writer owner

Owns declared-hash validation before cache lookup, temporary write, write-time verification, atomic publication through `mgc-platform`, corrupt-entry repair from already hash-verified input, and selected parent-directory fsync. A valid existing blob stays a dedup hit; non-hash verification errors remain fail-closed. The two methods shared with sibling import owners are visible only inside crate::cas::store::import. Diagnostics stay on shared StoreError.

Phụ trách kiểm tra hash khai trước cache lookup, ghi temp, xác minh khi ghi, publish nguyên tử qua `mgc-platform`, sửa entry hỏng từ input đã xác minh và fsync thư mục cha theo chế độ. Blob hợp lệ vẫn dedup; lỗi xác minh khác mismatch tiếp tục fail-closed. Hai method dùng chung với owner import khác chỉ thấy trong crate::cas::store::import. Diagnostic tiếp tục dùng StoreError chung.

Nearest source regression test: test/write_bytes.rs. The existing poisoned cache-hit attack case in adversarial_integrity.rs remains unchanged.
