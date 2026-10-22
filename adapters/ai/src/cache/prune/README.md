# AI cache prune

The public facade keeps `PruneStrategy`, `PruneResult`, `prune_cache` and `remove_model`. Candidate discovery, policy selection, byte measurement, entry removal, sidecar association/removal and typed errors have separate owners with nearest source tests.

Facade public giữ `PruneStrategy`, `PruneResult`, `prune_cache` và `remove_model`. Tìm candidate, chọn policy, đo byte, xóa entry, gắn/xóa sidecar và lỗi typed có owner riêng cùng test gần nhất.

For Hugging Face, model paths follow `models--...`, and prune/listing exclude `.locks`, version/control files, datasets and spaces. `KeepRecent` ranks readable entries by metadata `last_accessed`; unreadable metadata is retained. `LargerThan` measures logical content bytes, while other strategies avoid walking large model trees until an entry is actually removed. `.meta.json` sidecars are not candidates and uniquely-associated regular sidecars are removed with their entry. Logical byte totals do not promise physical disk reclamation. Entry symlinks are skipped; explicit `remove_model` refuses a symlink path.

Với Hugging Face, path model theo `models--...`; prune/list bỏ `.locks`, version/control, dataset và space. `KeepRecent` xếp entry đọc được theo `last_accessed`; metadata không đọc được được giữ lại. `LargerThan` đo logical content bytes; policy khác không duyệt cây model lớn cho tới khi entry thực sự được xóa. Sidecar `.meta.json` không phải candidate và sidecar thường gắn duy nhất được xóa cùng entry. Tổng logical byte không cam kết dung lượng vật lý được thu hồi. Symlink trong cache bị bỏ qua; `remove_model` từ chối path là symlink.

Source-level cases live under each owner's `test/` directory and the coordinator's `test/prune.rs`. They are not runtime evidence until the AI crate tests are executed.

Test source đặt trong `test/` của từng owner và `test/prune.rs` của coordinator. Đây chưa phải runtime evidence cho đến khi chạy AI crate tests.
