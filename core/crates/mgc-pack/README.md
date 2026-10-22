# mgc-pack

`mgc-pack` owns package archive preparation shared by MagiCore publish flows. It selects publishable files using npm-compatible ignore precedence, sanitizes package manifests, and writes deterministic streaming tarballs with digest metadata.

## Boundaries

- `src/ignore/` owns file selection. Its public route remains `mgc_pack::ignore::select_files`.
- `src/manifest/` owns the stable manifest facade and same-name type, sanitization, identity, dependency-projection and local-error owners.
- `src/tarball/` owns the stable `PackResult`/`pack` facade and same-name pack, entry-writer, digest, staging, type and local-error owners.
- Public result and error signatures remain the contract until a separately reviewed API change.

Tests are Cargo targets listed in `Cargo.toml`. The file-selection target is located beside its owner at `src/ignore/test/ignore.rs`; manifest owners have nearby unit-test files, while the six manifest integration cases remain under `tests/manifest.rs` to exercise the public facade. Tarball integration tests also remain under `tests/`.

Tarball package inputs use `mgc-platform::open_regular_file_no_follow` and metadata from the opened handle. This rejects final-component symlinks/reparse points; parent-directory components are not pinned and remain a documented race boundary. Runtime checks are pending. The archive digest reads the same staging handle later synced and persisted, avoiding temporary-path replacement between hash and publication.

## Bản tiếng Việt

`mgc-pack` sở hữu việc chuẩn bị archive package dùng chung cho các luồng publish của MagiCore. Crate chọn file theo precedence ignore tương thích npm, làm sạch manifest và ghi tarball xác định theo luồng kèm digest.

`src/ignore/` quản lý chọn file và giữ API `mgc_pack::ignore::select_files`; `src/manifest/` giữ facade manifest cùng các owner type/sanitize/identity/dependency/error; `src/tarball/` giữ facade `PackResult`/`pack` và tách owner pack, entry-writer, digest, staging, type cùng lỗi cục bộ. Integration target `test_ignore` nằm cạnh owner ở `src/ignore/test/ignore.rs`. Sáu integration case manifest vẫn ở `tests/manifest.rs` để kiểm tra facade công khai; integration tarball vẫn ở `tests/tarball.rs`, unit test hành vi nằm cạnh owner.

Input tarball dùng `mgc-platform::open_regular_file_no_follow` và lấy metadata từ handle đã mở. Cách này chặn symlink/reparse ở component cuối; component thư mục cha chưa được ghim và vẫn là ranh giới race được ghi nhận. Runtime check còn chờ. Digest archive cũng đọc chính handle staging được sync và persist, tránh thay pathname tạm giữa lúc hash và publish.
