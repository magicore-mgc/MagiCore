# Patch engine

`mod.rs` is the facade. `patches.rs` coordinates parse, path validation, hunk application and staged writes. Each behavior owner has its own error type, README and nearest tests. `types.rs` contains only the shared private Hunk data.

`mod.rs` là facade. `patches.rs` điều phối phân tích, kiểm tra đường dẫn, áp dụng hunk và ghi theo giai đoạn. Mỗi owner có error, README và test gần nhất; `types.rs` chỉ chứa dữ liệu Hunk riêng dùng chung.

The CLI records patch metadata but install application remains unsupported and fails closed. This module is UTF-8 text-only; it validates all files before the first write, but does not provide atomic multi-file writes and retains a path validation/open TOCTOU window.

CLI có ghi metadata patch nhưng install chưa hỗ trợ áp dụng và tiếp tục fail-closed. Module chỉ xử lý UTF-8; kiểm tra mọi file trước lần ghi đầu tiên nhưng chưa ghi nhiều file nguyên tử và còn cửa sổ TOCTOU giữa kiểm tra path với mở file.
