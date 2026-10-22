# CI template catalog / Danh mục template CI

## Responsibility / Trách nhiệm

Owns immutable release/checksum substitution and provider template routing. Each provider body is isolated as literal template data.

## Tests / Kiểm thử

Catalog and supply-chain tests inspect rendered content; they do not execute remote CI.

## Tiếng Việt

Owner này quản lý template bất biến, thay checksum/release pin và route tới template provider. Nội dung riêng từng provider nằm trong owner dữ liệu riêng. Test kiểm tra template đã render, không chạy CI từ xa.
