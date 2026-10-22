# Policy distribution owner / Owner phân phối policy

Read configured URL/OCI sources, require explicit trust roots, try URL first, fall back to OCI on URL failure and warn rather than fail the command. The shared typed distribution error preserves source chains; public transport functions keep `anyhow::Result`.

Owner này chỉ điều phối kênh và quản lý error contract dùng chung. Existing integration contract is tested in `cli/tests/policy_manifest_load.rs`; no empty owner-test directory is created.
