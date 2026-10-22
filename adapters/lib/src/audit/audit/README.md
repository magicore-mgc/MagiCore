# Lib audit dispatch / Điều phối audit Lib

Owns `run_audit` and the six `LibLanguage` branches. Python/Go/Java/.NET polyglot routes delegate to their plan owners; Rust and single-ecosystem lanes retain the existing scanner calls. TypeScript continues to return the existing delegation error.

Quản lý `run_audit` và sáu nhánh `LibLanguage`. Route polyglot Python/Go/Java/.NET giao cho owner plan tương ứng; Rust và lane một ecosystem giữ call scanner hiện tại. TypeScript tiếp tục trả lỗi yêu cầu delegate như cũ.

No new policy or report transformation is introduced. Direct dispatch/polyglot coverage is currently missing; add it in the later test phase. `cargo test -p mgc-lib-adapter` is the eventual crate check; it was not run during source-first work.

Không thêm policy hoặc bước biến đổi report. Hiện thiếu test trực tiếp dispatch/polyglot; bổ sung ở phase test sau. `cargo test -p mgc-lib-adapter` là lệnh crate check sau này; chưa chạy trong phase source-first.
