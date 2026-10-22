# Dart version constraints / Constraint version Dart

`version_constraints.rs` owns `dart_matches` and its comparison helpers. It returns a boolean and has no local error contract. The parent `pubdev` module is the only consumer export. Existing caret/exact/wildcard/range test cases are registered beside this owner.

The implementation's current caret behavior is intentionally preserved in this structural move; verify Dart zero-major caret semantics in a separately tracked behavior-fix batch.
