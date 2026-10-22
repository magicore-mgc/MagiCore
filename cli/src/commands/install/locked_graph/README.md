# Locked graph / Đồ thị từ lockfile

`locked_graph.rs` enforces regular non-symlink lockfiles, keeps v4 on the lossless shared path, checks legacy signatures and projects validated legacy packages into a resolved graph. Rival lockfiles without `mgc.lock` fail closed. Existing parser/trust errors keep their shared route; tests live under [`test`](test/README.md).

`locked_graph.rs` chặn lockfile không thường/symlink, chuyển v4 qua đường lossless dùng chung, xác minh chữ ký legacy và dựng graph từ package đã kiểm tra. Có lock đối thủ mà thiếu `mgc.lock` thì fail-closed. Test nằm cạnh owner.
