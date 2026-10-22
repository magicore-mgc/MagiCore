# Web entry-point discovery / Tìm entry point Web

Checks the established TypeScript/JavaScript candidates, then the `main` and `module` package fields, preserving current precedence. It does not generate files.

Kiểm tra các candidate TypeScript/JavaScript hiện có, sau đó tới field `main` và `module` của package, giữ nguyên precedence. Không sinh file.

`EntryPointError` represents the no-entry case; JSON/filesystem errors propagate with their source. Existing coverage is in the CLI build tests pending later relocation.

`EntryPointError` biểu diễn trường hợp không có entry; lỗi JSON/filesystem được propagate cùng source. Coverage hiện ở test build CLI, chờ chuyển sau.
