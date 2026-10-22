# Game cache / Cache Game

## Purpose / Mục đích

Owns local engine binary cache paths and cache lifecycle for game engines. Bevy does not use this binary cache.

Phụ trách đường dẫn cache binary engine và lifecycle cache. Bevy không dùng binary cache này.

## Errors and tests / Lỗi và kiểm thử

Uses shared platform path and `MgError`/`MgResult`; tests are next door in [`test/`](test/README.md).
