# Web scaffold processor

## Purpose

Owns native Web project materialization without template registry layers. `web.rs` is a thin facade exposing `WebProcessor`; `selection/` decides which sub-type/framework pairs are native, `project/` routes to a framework family, and `common/` owns package-name normalization plus baseline `.gitignore`/`README.md`. Family owners: `rust_backend/` (Axum, Actix Web), `python_backend/` (FastAPI, Flask), `go_backend/` (Echo, Fiber, Gin), `node_backend/` (Fastify, Hono, tRPC), `astro/` (frontend).

## Phạm vi

Owner này materialize project Web native không cần template layer registry. `web.rs` là facade mỏng expose `WebProcessor`; `selection/` quyết định cặp sub-type/framework nào chạy native, `project/` định tuyến theo họ framework, `common/` chuẩn hoá tên package và file nền `.gitignore`/`README.md`. Owner theo họ: `rust_backend/` (Axum, Actix Web), `python_backend/` (FastAPI, Flask), `go_backend/` (Echo, Fiber, Gin), `node_backend/` (Fastify, Hono, tRPC), `astro/` (frontend).
