# Policy manifest loading owner / Owner nạp manifest policy

The coordinator selects an active manifest only after user trust keys are available. `manifest_candidates/` reads sources in the established order: explicit user file, project `.magicore/policy`, then shared data-dir cache. `trust_keys/` parses only `MGC_POLICY_TRUST_KEYS`.

Owner này chọn manifest hợp lệ bằng verifier dùng chung. Invalid or absent input leaves native-deny active. DTOs remain owned by `mgc_exec`; this owner surfaces no new error type because failures are intentionally warnings or `None`.

Nearest tests cover candidate path precedence and trust-key parsing. Signed loading and project trust isolation remain covered by `cli/tests/policy_manifest_load.rs`.
