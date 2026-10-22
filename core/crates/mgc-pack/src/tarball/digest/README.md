# Archive digest / Digest archive

Owns fixed-size archive reads, SHA-1 hex shasum, SHA-512 base64 SRI and compressed byte count.

Quản lý lượt đọc archive bằng buffer cố định, shasum SHA-1 dạng hex, SRI SHA-512 base64 và số byte nén.

Digest reads the already-open staging handle used for final sync and publication; it does not reopen a temporary path.

Digest đọc handle staging đã mở, cũng là handle dùng để sync và publish; không mở lại pathname tạm.
