# CAS Integrity Snapshot

Provides a read-only, single-query summary of duplicate promoted generations and orphan CAS claims for diagnostics. The raw SQLite connection is not part of the downstream production API.

## Trách nhiệm

Cung cấp snapshot chỉ đọc bằng một truy vấn về generation promoted trùng và CAS claim mồ côi cho công cụ chẩn đoán. Connection SQLite thô không còn thuộc API production cho crate downstream.

`integrity_snapshot.rs` owns the query and public result; `error.rs` owns its typed SQLite failure. `mod.rs` only wires and exports the public types.

`integrity_snapshot.rs` quản lý truy vấn và kiểu kết quả public; `error.rs` quản lý lỗi SQLite có kiểu. `mod.rs` chỉ nối module và export type public.
