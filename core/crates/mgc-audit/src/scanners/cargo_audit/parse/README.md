# cargo-audit JSON compatibility parser

## Purpose

Preserves the public parser facade for cargo-audit JSON and coordinates strict required-field parsing, finding projection, severity selection, consistency checks and typed local parse errors.

## Phạm vi

Giữ facade parser public cho JSON cargo-audit và điều phối parse field bắt buộc, ánh xạ finding, chọn severity, kiểm tra consistency và lỗi parse typed cục bộ.

This parser is a compatibility API, not the product Rust audit path. Production Rust audit uses the separate native `audit_rust` scanner.
