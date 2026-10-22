# Joint-root graph resolution / Resolve graph nhiều root

`graph_resolution.rs` owns iterative constraint rebuilding, concurrent metadata fetch bounded by the shared resolver limit, stable selection signatures, oscillation detection and the maximum pass count. `error.rs` owns graph-specific typed failures and maps them to current `MgError` categories. `mod.rs` is wiring/re-export only.

Nearest unit checks cover stable signatures and local error mapping. `core/crates/mgc-resolver/tests/protocols_pub.rs` covers joint-root intersections and bounded concurrent requests. Runtime remains NOT_VERIFIED until tests run.
