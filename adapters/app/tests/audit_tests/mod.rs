//! App audit test-owner wiring and shared test fixture export.
//! Nối các owner test audit App và xuất fixture dùng chung trong test crate.

mod flutter_audit;
mod kotlin_audit;
mod manifest_aggregation;
mod native_lock_parsers;
mod swift_project_scope;
mod test_support;
mod unsupported_framework_audit;

pub(crate) use test_support::tmp;
