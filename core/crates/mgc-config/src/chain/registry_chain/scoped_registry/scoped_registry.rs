//! Registry paired with the scope permitted to carry its credential.
//! Registry đi cùng phạm vi được phép mang credential.

use crate::registry::Registry;

/// Registry and credential scope travel together; tokens are never Debug output.
/// Native transport phải giữ scope tới request; không dùng API value-only để wiring.
pub struct ScopedRegistry {
    pub registry: Registry,
    pub credential_scope: Option<url::Url>,
}
