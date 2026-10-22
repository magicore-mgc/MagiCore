# Resolution / Điều phối resolve

Owns the dependency-resolution flow: request preparation, v4 and legacy lock/cache short-circuits, offline fail-closed gate, solver, metadata/peer validation, package graph assembly, supply-chain checks and resolution-cache publication. Compatibility helper exports keep existing in-crate test paths stable.

Sở hữu luồng resolve dependency: chuẩn bị request, short-circuit lock/cache v4 và legacy, cổng offline fail-closed, solver, kiểm metadata/peer, lắp graph, guard supply-chain và ghi resolution-cache. Re-export helper tương thích giữ nguyên đường dẫn test nội bộ. `resolution.rs` remains a coordinator and is scheduled for a finer source-only split in the next batch.
