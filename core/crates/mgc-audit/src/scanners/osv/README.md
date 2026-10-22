# OSV audit owners

This folder decomposes the OSV.dev scanner into bounded query, severity, aggregation, Swift Package Manager, CocoaPods, and shared wire-type owners. `mod.rs` is wiring and compatibility exports only; `scanners::OsvPin` and every prior scanner/parser function remain available at their existing paths.

## Phạm vi

Folder này tách scanner OSV.dev thành các owner truy vấn có giới hạn, ánh xạ severity, tổng hợp kết quả, Swift Package Manager, CocoaPods và wire type dùng chung. `mod.rs` chỉ làm nhiệm vụ nối/export tương thích; `scanners::OsvPin` và các hàm scanner/parser cũ giữ nguyên đường dẫn.

The implementation keeps `mgc_types::MgError` and `MgResult` as the shared error contract. It does not add duplicate local error types, change retry behavior, weaken the fail-closed report, or mark unsupported CocoaPods as clean.
