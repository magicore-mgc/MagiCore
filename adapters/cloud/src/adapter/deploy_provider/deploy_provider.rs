//! Keeps deploy unclaimed and inherits the shared fail-closed unsupported probe. — Giữ deploy chưa được claim và kế thừa probe unsupported fail-closed dùng chung.

use mgc_types::capabilities::DeployProvider;

use super::super::CloudAdapter;

impl DeployProvider for CloudAdapter {
    // Inherit the default Unsupported probe until a native deploy engine exists.
}
