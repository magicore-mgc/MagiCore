# Go OSV fallback

## Purpose

Queries only pinned go.mod require entries through the shared OSV client and retains Partial coverage for the unscanned transitive graph.

## Phạm vi

Chỉ truy vấn require pin trong go.mod qua OSV client chung và giữ trạng thái Partial vì chưa quét dependency bắc cầu.
