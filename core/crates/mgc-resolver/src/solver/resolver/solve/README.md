# Shared resolver solve / Luồng solve dùng chung

This folder owns the dependency-solver loop shared by resolver clients. `solve.rs` keeps `Resolver::solve -> SolveResult` stable and adds `solve_with_roots -> SolveResultWithRoots`; indexed root provenance is retained for fresh, overridden and already-resolved requests. Focused owners handle batch version selection, dependency memo/prefetch, resolution materialization, and peer-instance merging.

Folder này sở hữu vòng resolve dependency dùng chung cho các adapter. `solve.rs` giữ contract `Resolver::solve -> SolveResult`, đồng thời thêm `solve_with_roots -> SolveResultWithRoots`; provenance root theo index được giữ cho request mới, override và request dùng lại version đã resolve. Owner riêng xử lý chọn version theo lô, memo/prefetch dependency, materialize resolution và gộp peer instance.

`resolution/` owns dependency enqueue and peer merge. `types/` contains solve-local aliases and indexed queue provenance. Public result types live under `resolver/types/`. `SolveError` remains the existing shared contract; this refactor does not introduce a duplicate error type. Existing Cargo integration tests stay at their current target and are not run in this source-first phase.
