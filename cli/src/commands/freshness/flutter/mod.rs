//! Flutter owner exports — export owner flutter.

mod flutter;
pub(super) use flutter::check_flutter_manifest_freshness;
pub use flutter::{check_flutter_freshness, declared_flutter_deps};
