# Audit integration tests

## Purpose

This directory keeps crate-level integration targets for audit scanners. The aggregation contract target is located beside its implementation at src/aggregate/test/aggregate.rs and explicitly registered as aggregate_test in Cargo.toml. The govulncheck suite retains its target and harness in govulncheck_test.rs, with fixture and behavior groups under govulncheck/.

## Phạm vi

Thư mục này giữ các integration target cấp crate cho audit scanner. Target contract tổng hợp nằm cạnh implementation tại src/aggregate/test/aggregate.rs và được đăng ký tường minh với tên aggregate_test trong Cargo.toml. Bộ govulncheck giữ target/harness tại govulncheck_test.rs, còn fixture và nhóm hành vi nằm trong govulncheck/.
