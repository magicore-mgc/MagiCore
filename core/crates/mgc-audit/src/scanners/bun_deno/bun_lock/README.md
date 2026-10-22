# Bun lockfile parser

English: Parses Bun lockfile JSONC, reads only the first string from each package array, and preserves the existing malformed-shape errors. Other identifiers remain in `jsr_skipped`. JSONC normalization is shared with `mgc-types`; this owner makes no network request.

Tiếng Việt: Phân tích JSONC của lock Bun, chỉ đọc chuỗi đầu tiên trong mỗi mảng package và giữ nguyên lỗi shape sai hiện có. Identifier khác được ghi vào `jsr_skipped`. Chuẩn hóa JSONC dùng chung với `mgc-types`; owner này không gọi mạng.
