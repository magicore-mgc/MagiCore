# Cache prune policy

This owner maps `PruneStrategy` to candidate indexes. Age/access strategies require readable metadata; unknown metadata is retained. `KeepRecent` ranks `last_accessed` newest first and uses path order to break ties. `LargerThan` compares measured content bytes and does not count metadata sidecars toward its threshold.

Owner này chuyển `PruneStrategy` thành danh sách candidate index. Policy tuổi/truy cập cần metadata đọc được; metadata không rõ thì giữ entry. `KeepRecent` xếp `last_accessed` mới nhất trước, dùng path để xử lý tie. `LargerThan` so theo content bytes đã đo và không tính sidecar vào ngưỡng.

Nearest policy cases are in `test/policy.rs`; they are source-only until runtime verification runs.

Test policy gần owner nằm tại `test/policy.rs`; hiện là source-only cho đến khi chạy runtime verification.
