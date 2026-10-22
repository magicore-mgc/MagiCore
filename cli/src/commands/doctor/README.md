# Doctor command

This module implements the shared `mgc doctor check` command. `mod.rs` preserves the public `commands::doctor` API while `doctor.rs` coordinates focused owners for CLI options, report data, toolchain discovery, store/disk inspection, registry reachability, diagnosis and output.

Store inspection does not create the configured global store. For an existing directory it creates a private temporary write probe and checks that cleanup succeeds. A process crash can leave the probe behind; a cleanup error is reported with its path. Free space is queried through the nearest existing directory ancestor when the store path is missing, and low-space guidance names the directory actually queried.

Store diagnostics now distinguish a missing path, a dangling symbolic link anywhere in the store path, a non-directory path, a failed write probe and unknown inspection/disk state. A dangling link is not offered for automatic repair. The former `ERR_STORE_READONLY` code is replaced by the more precise `ERR_STORE_NOT_WRITABLE` for failed probes; consumers matching exact codes should handle the new statuses.

Toolchain discovery keeps its closed six-entry list, resolves regular files from OS-parsed PATH, pins relative parents and sends the exact path plus fixed argv through the MGC executor. Python uses `-S --version` to skip `sitecustomize`; Rust uses the policy-approved `-vV`. Probes have a five-second timeout, process-tree supervision and bounded output. MGC clears the child environment, then restores guarded PATH, Windows-required process variables and a short UTF-8 allowlist for home/toolchain-manager selectors; Node/Python ambient startup hooks are omitted. A custom shim needing another environment variable can be found but report an unavailable version. Non-Unicode executable paths are discovered but cannot pass through the current string-based executor. The executor does not sandbox a selected PATH binary; it runs with the current user's permissions. Visible paths escape terminal controls. A same-user replacement race between path validation and OS process opening remains possible.

`mgc doctor check --fix` prepares missing parent directories and atomically creates the store leaf only when it is missing, then uses the verified post-action snapshot in the report. If the leaf appears during repair, doctor inspects it without replacing it. A valid symlink to a writable directory remains accepted as an existing store and receives the same transient write probe; dangling links are skipped. It does not change permissions or replace an existing file/path. Both text and `--json --fix` report the remediation outcome; JSON without `--fix` retains its current report fields and has no `remediation` key.

With `--fix`, the process exit code reports whether that store repair completed; it does not replace reading `health_status` for unrelated doctor findings. Without `--fix`, diagnostics themselves do not change the command exit code; internal execution or serialization errors still do.

# Lệnh doctor

Module này triển khai lệnh dùng chung `mgc doctor check`. `mod.rs` giữ API công khai `commands::doctor`; `doctor.rs` điều phối các owner nhỏ cho cờ CLI, dữ liệu report, tìm toolchain, kiểm tra store/disk, reachability registry, chẩn đoán và output.

Kiểm tra store không tạo global store đã cấu hình. Với thư mục có sẵn, doctor tạo file thử quyền ghi riêng tư rồi xác nhận xóa được. Process crash có thể để lại file thử; lỗi dọn được báo kèm path. Nếu store chưa có, dung lượng trống được truy vấn qua thư mục tổ tiên gần nhất đang tồn tại; hướng dẫn low-space nêu directory thực tế được hỏi.

Diagnostic store giờ phân biệt path thiếu, symlink bị đứt trên bất kỳ thành phần nào của đường dẫn store, path không phải thư mục, write probe thất bại và trạng thái inspection/disk chưa biết. Symlink bị đứt không được tự động sửa. Code cũ `ERR_STORE_READONLY` được thay bằng `ERR_STORE_NOT_WRITABLE` chính xác hơn khi probe thất bại; consumer so khớp exact code cần xử lý các trạng thái mới.

Tìm toolchain giữ danh sách đóng sáu mục, phân giải file thường từ PATH qua API OS, ghim parent tương đối rồi truyền đúng path cùng argv cố định qua executor MGC. Python dùng `-S --version` để bỏ `sitecustomize`; Rust dùng `-vV`. Probe có timeout 5 giây, được giám sát cây process và giới hạn output. MGC xóa environment child rồi khôi phục PATH đã guard, biến Windows bắt buộc và allowlist UTF-8 ngắn cho HOME/toolchain manager; startup hook Node/Python ambient bị loại. Shim tùy biến cần biến ngoài allowlist có thể được tìm thấy nhưng trả `?`. Tool đã tìm thấy nhưng probe lỗi/rỗng sẽ hiện `?` và có warning. Path executable không Unicode được phát hiện nhưng executor string hiện tại không nhận để chạy. Executor không sandbox binary PATH đã chọn; process chạy với quyền user hiện tại. Path hiển thị escape ký tự điều khiển. Race cùng user thay file giữa lúc xác thực path và lúc OS mở process chưa bị chặn nguyên tử.

`mgc doctor check --fix` chuẩn bị parent directory còn thiếu rồi tạo leaf store bằng thao tác nguyên tử khi nó chưa tồn tại; sau đó dùng snapshot đã xác minh để lập report. Nếu leaf xuất hiện trong lúc sửa, doctor kiểm tra lại nhưng không thay thế nó. Symlink hợp lệ trỏ tới thư mục ghi được vẫn được chấp nhận là store hiện hữu và nhận cùng write probe tạm; symlink bị đứt được bỏ qua. Lệnh không đổi permission hoặc thay thế file/path đã tồn tại. Cả text và `--json --fix` đều báo kết quả remediation; JSON không có `--fix` giữ nguyên các field report hiện tại và không có key `remediation`.

Khi có `--fix`, exit code cho biết thao tác sửa store có hoàn tất hay không; nó không thay cho việc đọc `health_status` để xử lý finding doctor khác. Không bật `--fix` thì diagnostic tự nó không đổi exit code; lỗi thực thi hoặc serialization vẫn làm command thất bại.

Toolchain probe tests are authored but remain NOT_RUN.

Source test của toolchain probe đã viết nhưng còn NOT_RUN.
