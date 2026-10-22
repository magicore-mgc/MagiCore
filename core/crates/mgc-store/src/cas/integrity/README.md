# CAS integrity

This module owns the typed BLAKE3 digest API, strict foreign-hash validation and the tarball-entry DTO. `mod.rs` only preserves and re-exports the former `cas::integrity` API. Digest construction, invalid-hash errors and import DTOs have separate owners. Input/display resource hardening remains an open follow-up; the source move preserves current behavior.

# CAS integrity — trách nhiệm

Module này sở hữu API digest BLAKE3 typed, validation hash ngoài nghiêm ngặt và DTO entry tarball. `mod.rs` chỉ giữ rồi export lại API `cas::integrity` cũ. Dựng digest, lỗi hash sai và DTO import có owner riêng. Giới hạn input/hiển thị để harden còn là việc tiếp theo; lần di chuyển source giữ behavior hiện tại.
