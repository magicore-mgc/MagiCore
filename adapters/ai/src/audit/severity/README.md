# severity

EN: Defines the ordered severity values used by AI model audit findings.

VI: Khai báo các mức độ ưu tiên dùng trong finding audit model AI.

- API: re-exported as `audit::Severity`, preserving its public path and ordering.
- Invariant: derived ordering remains `Info < Low < Medium < High < Critical`.
- Errors: data-only enum; no error owner is needed.
- Extension: add a level only with a reviewed compatibility change to consumers and thresholds.
- Tests: `test/severity_ordering.rs` retains the original ordering assertions; runtime remains NOT_RUN.
