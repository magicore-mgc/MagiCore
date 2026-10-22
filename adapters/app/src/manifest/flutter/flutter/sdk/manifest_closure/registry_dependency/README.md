# registry_dependency

EN: Parses a Flutter SDK package's registry constraint, safely intersects it with project/SDK ranges, and records the resulting pub dependency.

VI: Đọc constraint registry từ package Flutter SDK, giao an toàn với range của dự án/SDK và ghi dependency pub sau cùng.

- Input/output: package/dependency identity, YAML constraint, dev edge and prior range state → updated manifest and range state.
- Invariants: unsupported constraints and required disjunctive intersections fail closed; runtime edges promote a project dev-only dependency.
- Errors: typed `FlutterError` values preserve existing CLI/adapter mapping; this owner adds no retry behavior.
- Extension: range merge/grouping rules belong here; breadth-first SDK traversal belongs to `dependency_closure`.
- Tests: `../test/` covers promotion and disjunction; tests remain `NOT_RUN`.
