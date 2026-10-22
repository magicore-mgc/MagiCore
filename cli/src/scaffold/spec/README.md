# scaffold spec

This owner defines the shared, core-neutral scaffold request model and keeps the existing `scaffold::spec` public paths stable through `mod.rs` re-exports. Parsing, reference validation, core labels, normalization, artifact naming, and typed local failures each have a named owner below.

Owner này định nghĩa model yêu cầu scaffold dùng chung, không phụ thuộc core; các public path `scaffold::spec` được giữ ổn định qua re-export trong `mod.rs`. Parser, reference, nhãn core, chuẩn hóa tên, artifact naming và lỗi typed có owner riêng.

The parser currently accepts the existing heuristic only: a numeric first reference character selects `Version`, `^`/`~` selects `Range` (which validation rejects), and other references become `DistTag`. Name normalization lowercases, replaces `_` with `-`, then trims. Do not infer full SemVer validation from this module.

Parser hiện giữ nguyên heuristic: reference bắt đầu bằng số thành `Version`, `^`/`~` thành `Range` (bị validation từ chối), trường hợp khác thành `DistTag`. Tên được lowercase, thay `_` bằng `-`, rồi trim. Không xem đây là kiểm tra SemVer đầy đủ.

Focused tests live beside their implementation under each `test/` directory. They are authored but remain `NOT_RUN` until the source-first refactor phase is complete.
