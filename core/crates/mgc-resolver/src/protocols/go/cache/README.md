# Go cache owner

Validates module/version path coordinates, applies Go proxy uppercase escaping, and materializes `.zip`, `.mod`, `.info`, and Go `h1:` `.ziphash` files into the download cache.

## Tiếng Việt

Kiểm tra an toàn tọa độ module/version, escape chữ hoa theo Go proxy và materialize `.zip`, `.mod`, `.info`, `.ziphash` `h1:` đúng cache layout. Việc hash/parse zip xảy ra trước khi tạo file cache, như trước refactor. Unit test path escape nằm trong `test/`; integration materialization tests giữ ở suite resolver.
