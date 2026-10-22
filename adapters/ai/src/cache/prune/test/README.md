# Prune coordinator tests

These cases exercise the public prune facade across owner boundaries: age-based deletion, `KeepRecent`, corrupt metadata retention, size thresholds, path traversal rejection and sidecar cleanup. Focused owner policy/filesystem cases remain beside their modules.

Các test này kiểm tra facade prune qua nhiều owner: xóa theo tuổi, `KeepRecent`, giữ metadata hỏng, ngưỡng size, từ chối path traversal và dọn sidecar. Test policy/filesystem tập trung nằm cạnh module tương ứng.

All cases are source-only until executed by the AI adapter test target.

Toàn bộ test hiện chỉ mới có source, chưa được chạy bằng AI adapter test target.
