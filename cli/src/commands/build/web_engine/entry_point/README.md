# Web entry-point discovery / Tìm entry point Web

Resolves a Web build entry in a documented precedence order:

1. Conventional browser entries from the framework templates (`src/index.ts`, `src/main.tsx`, `index.js`, `main.js`, …).
2. Explicit `package.json` fields, in order `main`, `module`, `bin` (string or object form). A declared path that does not exist falls through.
3. Conventional Node server entries (`server.js`, `server.mjs`, `server.cjs`, `app.js`, `index.js`, `src/server.js`) for generators that ship no `src/` (Express and hand-written Node servers).

It does not generate files.

Thứ tự ưu tiên tường minh: (1) entry browser quy ước từ template framework; (2) field `main`/`module`/`bin` khai trong `package.json` (dạng chuỗi hoặc object; path khai không tồn tại sẽ bỏ qua); (3) entry server Node quy ước (`server.js`, `app.js`, …) cho generator không có `src/` như Express. Không sinh file.

`EntryPointError` represents the no-entry case; JSON/filesystem errors propagate with their source. Unit coverage lives in `test/entry_point.rs` (offline filesystem fixtures, no package manager, no network).

`EntryPointError` biểu diễn trường hợp không có entry; lỗi JSON/filesystem được propagate cùng source. Test unit ở `test/entry_point.rs` (fixture filesystem ngoại tuyến, không package manager, không mạng).
