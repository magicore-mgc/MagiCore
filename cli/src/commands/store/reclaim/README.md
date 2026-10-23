# Store reclaim

## Purpose

Owns `mgc store reclaim`: lists packages no project references, with total size, and deletes them only after `--apply` then `--yes`. It wraps the existing prune planner so both paths share one safety check.

## Vietnamese

Sở hữu `mgc store reclaim`: liệt kê package không project nào tham chiếu kèm tổng dung lượng, chỉ xoá sau `--apply` rồi `--yes`. Lệnh bọc planner prune hiện có để hai đường dùng chung một bước kiểm an toàn.
