# Verified Blob Owner

**Purpose:** Verify a blob against its full CAS address and use only the bounded, identity-checked Unix memo.

**Mục đích:** Xác minh blob theo địa chỉ CAS đầy đủ và chỉ dùng memo Unix có giới hạn, được kiểm tra identity.

`verified_blob.rs` preserves first-touch rehash, full digest plus executable memo keys, hit/miss/eviction accounting, mismatch invalidation, and quarantine. `open_blob/` owns memo-gated verification of an already-open descriptor and returns that exact descriptor rewound for the CAS export consumer. The local test owners cover memo and handle behavior. `mod.rs` only declares owners.

`verified_blob.rs` giữ nguyên rehash lần đầu, key memo gồm digest và executable, bộ đếm hit/miss/eviction, xóa memo khi lệch và cách ly. `open_blob/` quản lý xác minh memo trên descriptor đã mở và trả lại đúng descriptor đó, rewind để export CAS dùng tiếp. Các owner test cục bộ kiểm tra memo và hợp đồng handle. `mod.rs` chỉ khai báo owner.
