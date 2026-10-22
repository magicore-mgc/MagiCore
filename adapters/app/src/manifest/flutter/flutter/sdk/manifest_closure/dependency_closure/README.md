# dependency_closure

EN: Orchestrates breadth-first traversal of Flutter SDK package dependencies and returns the reachable SDK package set plus the expanded native pub manifest.

VI: Điều phối duyệt rộng các dependency package Flutter SDK, trả tập package SDK đạt tới và manifest pub native đã mở rộng.

- Inputs: project root, selected SDK root, base manifest and declared SDK package/dev edges.
- Invariants: names are validated before traversal; each SDK package is processed according to its strongest runtime/dev edge; SDK packages remain SDK-provided, while registry packages enter the pub graph through the range owner.
- Error policy: canonicalization, unsupported SDK source and malformed metadata fail through existing `FlutterError`; no error is swallowed.
- Extension: traversal/deduplication belongs here; per-package filesystem validation and range policy remain delegated to their owners.
- Tests: `../test/sdk_dependency_graph.rs`, `runtime_dev_promotion.rs`, malformed and range cases. Tests were relocated but not run.
