# Cache metadata sidecar owner

This owner maps a model path to its `.meta.json` sidecar, detects sibling-path collisions, and deletes only a regular sidecar. Prune batches freeze unique ownership during candidate discovery; explicit single-model removal checks siblings immediately before deletion.

Owner này map model path sang sidecar `.meta.json`, phát hiện collision giữa sibling và chỉ xóa sidecar file thường. Prune batch đóng băng ownership duy nhất lúc discovery; xóa một model riêng kiểm tra sibling ngay trước khi xóa.

Nearest behavior and filesystem-boundary cases are in `test/sidecar.rs`.

Test hành vi và ranh giới filesystem gần owner nằm tại `test/sidecar.rs`.
