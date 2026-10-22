# CAS Verification Owners

**Purpose:** Keep path validation, bounded memo checks, quarantine, membership, and external-hash validation behind the stable `cas::store` API.

**Mục đích:** Gom owner kiểm tra path, memo có giới hạn, cách ly, membership và xác thực hash ngoài sau API ổn định `cas::store`.

## Owner map

- `verify.rs` streams a regular file through BLAKE3 and rejects links before opening it.
- `open_file/` maps the shared `mgc-platform` no-follow opener to the CAS diagnostic contract.
- `verified_blob/` checks the complete CAS address, file identity, memo counters, and mismatch behavior.
- `file_identity/` provides Unix identity and disables memoization where no supported identity exists.
- `quarantine/` moves corrupt blobs and writes forensic reports best-effort.
- `membership/` owns `contains` and `remove`, including memo invalidation.
- `external_hash/` exports the existing external BLAKE3 preflight.
- `test/` covers streaming and path rejection; each child owner keeps its focused tests nearby.

The `mod.rs` files only wire these owners. `verify` and `verify_stored_blob` read/hash the same handle used for file-type and memo-identity checks. Public and crate-local API boundaries, diagnostic strings, hash behavior, and platform gates are preserved. Parent-component containment and concurrent in-place mutation remain separate security boundaries. Runtime tests are intentionally pending until the repository source pass closes.

## Sơ đồ owner

- `verify.rs` hash file thường theo luồng BLAKE3 và từ chối link trước khi mở file.
- `open_file/` ánh xạ helper no-follow của `mgc-platform` vào hợp đồng diagnostic CAS.
- `verified_blob/` kiểm tra đầy đủ địa chỉ CAS, file identity, bộ đếm memo và hành vi khi hash lệch.
- `file_identity/` cung cấp identity trên Unix và tắt memo ở nền tảng chưa có identity được hỗ trợ.
- `quarantine/` di chuyển blob hỏng và ghi báo cáo điều tra theo kiểu best-effort.
- `membership/` sở hữu `contains`, `remove` và vô hiệu hóa memo.
- `external_hash/` export preflight BLAKE3 ngoài hiện có.
- `test/` kiểm tra streaming/path; owner con giữ bộ test tập trung gần mã.

Các `mod.rs` chỉ nối module. `verify` và `verify_stored_blob` đọc/hash cùng handle đã dùng để kiểm tra file type và memo identity. Giữ nguyên ranh giới API, thông báo lỗi, cách hash và cfg theo nền tảng. Giới hạn component cha và ghi đồng thời nội dung file vẫn là ranh giới security riêng. Test runtime chờ đến khi hoàn tất lượt refactor source toàn repo.
