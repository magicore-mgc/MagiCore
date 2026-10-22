//! Embedded client protocol string test.
//! Test string protocol client nhúng.

use super::HMR_CLIENT_SCRIPT;

#[test]
fn embeds_the_existing_connected_reload_and_endpoint_strings() {
    assert!(HMR_CLIENT_SCRIPT.contains("data.type === 'connected'"));
    assert!(HMR_CLIENT_SCRIPT.contains("data.type === 'reload'"));
    assert!(HMR_CLIENT_SCRIPT.contains("/@magicore/hmr`"));
}
