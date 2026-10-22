# Low disk space

This doctor sub-owner contains the named warning threshold and pure low-space advice. A zero free-byte reading means disk availability is unknown and does not warn. Advice names both the configured global MGC store and the exact existing directory used for the volume-space query, so a missing store does not imply a mount that has not been verified. It does not suggest project-local `mgc store prune` as if that command cleaned the global store. `analysis/store_issues` maps the advice into the `DiagnosticIssue` schema.

# Dung lượng đĩa thấp

Owner con của doctor này chứa ngưỡng cảnh báo có tên và hàm hướng dẫn thuần. Số byte trống bằng 0 nghĩa là chưa biết dung lượng, không phát cảnh báo low-space. Hướng dẫn nêu cả store global đã cấu hình và directory hiện hữu chính xác đã dùng để truy vấn volume; store thiếu không bị coi như một mount đã xác minh. Owner không gợi ý `mgc store prune` theo project như thể lệnh đó dọn global store. `analysis/store_issues` ánh xạ kết quả vào schema `DiagnosticIssue`.
