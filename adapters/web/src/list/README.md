# Web installed-package listing

## Purpose

Owns `run_list`: reads the manifest, prefers the verified V4 graph (never parsed through the legacy reader), and reports installed packages only when the materialized `package.json` proves a version.

## Phạm vi

Sở hữu `run_list`: đọc manifest, ưu tiên graph V4 đã verify (không đi qua reader legacy), và chỉ báo package đã cài khi `package.json` đã materialize chứng minh được version.

## Tests

`test/list.rs` covers every manifest section (`dependencies`, `devDependencies`, `optionalDependencies`, `peerDependencies`), the declared-but-not-installed case, and a regression where an unverifiable `node_modules/<name>` directory used to abort the whole listing.
(Liệt kê đủ 4 section manifest, trường hợp khai nhưng chưa cài, và hồi quy khi thư mục không xác minh được từng làm hỏng cả lần liệt kê.)
