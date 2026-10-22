# bench_matrix

Cargo auto-discovers this benchmark binary from the sibling bench_matrix.rs. This folder owns the CLI, typed benchmark data, scenario workloads, fixture generation, cache setup, filesystem metrics, baseline I/O and reports.

Cargo tự phát hiện binary benchmark từ file bench_matrix.rs cùng cấp. Folder này sở hữu CLI, type benchmark, workload, fixture, cache setup, metric filesystem, baseline và báo cáo.

The six scenarios and their order are coordinated in matrix/. Each workload has its own folder. mod.rs files only wire modules and exports. The existing environment-based cache isolation currently mutates process environment while a multi-thread Tokio runtime exists; see docs/ProgressReport.md. Runtime verification remains pending.

Sáu scenario và thứ tự chạy nằm ở matrix/. Từng workload có folder riêng. mod.rs chỉ nối module/export. Luồng cô lập cache hiện vẫn sửa process environment khi Tokio runtime đa luồng đã tồn tại; xem docs/ProgressReport.md. Chưa xác minh runtime.
