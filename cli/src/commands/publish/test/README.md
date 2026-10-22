# publish test owners

This folder owns unit tests for `cli/src/commands/publish/`. `mod.rs` keeps the tests under the existing private `tests` module and provides the shared imports needed by the child files.

- `registry_policy.rs` covers registry argument wiring, canonical OCI repository names, token deadlines, upload-location validation and artifact-manifest metadata.
- `git_checks.rs` covers the closed read-only Git query set, selected project root, lag-count parsing and rejection of a branch behind its upstream.
- `lifecycle_policy.rs` covers publish-hook opt-in, `--ignore-scripts` and malformed lifecycle metadata.
- `trusted_oci_upload.rs` owns the local mock OCI registry and its single streamed-upload test.

These files preserve existing test source. They do not prove that the tests compile or pass; test execution remains `NOT_RUN` in this source-first phase.

# publish test owners (VI)

Folder này sở hữu unit test cho `cli/src/commands/publish/`. `mod.rs` giữ test bên trong module riêng tư `tests` hiện có và cung cấp import dùng chung cho các file con.

- `registry_policy.rs` kiểm tra cấu hình đối số registry, tên repository OCI chuẩn hóa, hạn token, xác thực upload URL và metadata manifest.
- `git_checks.rs` kiểm tra tập lệnh Git chỉ đọc, project root được chọn, parse số commit chậm và chặn branch đứng sau upstream.
- `lifecycle_policy.rs` kiểm tra opt-in hook publish, `--ignore-scripts` và metadata lifecycle sai định dạng.
- `trusted_oci_upload.rs` sở hữu mock OCI registry cục bộ và test upload dạng stream duy nhất.

Các file giữ nguyên test source cũ. Việc tách file không chứng minh test biên dịch hoặc pass; test vẫn `NOT_RUN` trong giai đoạn refactor source này.
