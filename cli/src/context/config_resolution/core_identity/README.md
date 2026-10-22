# Core identity owner / Owner identity core

`core_identity.rs` normalizes core identifiers through `Ecosystem`, rejects a different owner with a typed local error, and derives a fallback project name from its root path.

`core_identity.rs` chuẩn hóa tên core qua `Ecosystem`, từ chối owner khác bằng lỗi typed cục bộ và lấy tên mặc định từ đường dẫn root.

Nearest tests cover the supported `clo` alias and typed mismatch. Test source is not yet run.

Test gần owner kiểm tra alias `clo` và lỗi mismatch typed. Test chưa chạy.
