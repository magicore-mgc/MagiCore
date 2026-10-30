# Rust online install / Install Rust online

This owner downloads missing registry archives, verifies registry integrity, imports verified bytes into the MagiCore CAS, and materializes Cargo-compatible source/cache entries with checksum metadata. It keeps the existing sequential order and does not invoke Cargo as an installer.

Owner này tải archive registry còn thiếu, xác minh integrity, import bytes hợp lệ vào CAS MagiCore và materialize source/cache Cargo cùng checksum metadata. Luồng tuần tự hiện tại được giữ nguyên và không gọi Cargo làm installer.

EN: After materializing, the `source_readiness` owner writes the checksum record and Cargo's `.cargo-ok` marker; a source that already verifies but lacks the marker (older MagiCore store) gets the marker repaired instead of being rebuilt.

VI: Sau khi materialize, owner `source_readiness` ghi checksum record và marker `.cargo-ok` của Cargo; source đã hợp lệ nhưng thiếu marker (store MagiCore cũ) được vá marker thay vì dựng lại.
