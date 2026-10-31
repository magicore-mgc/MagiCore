# Web dev backend-half owner

Resolves how the `server/` half of a split fullstack project starts. A Node starter keeps a named `package.json` script so the whole launch path (flag policy, optimizer env, audit log, hooks) still applies; every other generated backend resolves to its own mature runner (`python3 main.py`, `python3 manage.py runserver`, `cargo run --quiet`, `go run .`), configured from the process environment. No package manager is spawned and nothing is installed implicitly.

Resolve cách nửa `server/` của project fullstack tách rời khởi động. Starter Node giữ script `package.json` có tên để toàn đường launch (policy flag, env optimizer, audit log, hook) vẫn áp dụng; mọi backend sinh ra khác resolve về runner chín của chính nó (`python3 main.py`, `python3 manage.py runserver`, `cargo run --quiet`, `go run .`), cấu hình từ môi trường process. Không spawn package manager và không cài ngầm.
