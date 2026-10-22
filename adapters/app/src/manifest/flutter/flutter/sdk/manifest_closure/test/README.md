# manifest_closure tests

EN: Nearest-owner tests for declared SDK dependencies, transitive package traversal, group promotion, malformed metadata, constraints and override semantics.

VI: Test gần owner nhất cho dependency SDK khai báo, duyệt package bắc cầu, promote nhóm, metadata sai, constraint và override.

Each legacy test item is kept in a subject file and registered by `mod.rs`. The tests use the adapter's existing Flutter `FlutterError` boundary. S274 has not run them; every runtime result remains `NOT_RUN` until the repository source pass closes.
