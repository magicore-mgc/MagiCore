# Go offline environment / Môi trường Go offline

## Responsibility / Trách nhiệm

Builds the fixed `GOPROXY=off`, `GOSUMDB=off`, `GOTOOLCHAIN=local` environment passed to the Go compiler route. Dependency acquisition and verification must already have been performed by MagiCore.

Dựng environment cố định `GOPROXY=off`, `GOSUMDB=off`, `GOTOOLCHAIN=local` cho route Go compiler. MGC phải lấy và xác minh dependency trước đó.

## Errors and tests / Lỗi và test

Environment construction is infallible and owns no local error type. Coverage is part of the build command test suite; no tests were run during this source-first pass.

Dựng environment không phát sinh lỗi và không cần error type riêng. Coverage thuộc bộ test lệnh build; lượt source-first này chưa chạy test.
