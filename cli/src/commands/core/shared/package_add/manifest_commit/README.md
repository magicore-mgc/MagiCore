# Add manifest commit

Owns the current journal stage, manifest write and post-image sequence. It returns whether this operation acquired a journal so the caller can choose the existing finish/rollback path.

## Tiếng Việt

Owner này giữ thứ tự stage journal, ghi manifest và ghi post-image hiện có. Hàm trả trạng thái operation có journal để caller chọn nhánh finish/rollback đang dùng.
