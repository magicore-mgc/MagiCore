//! Stable native engine API path test.
//! Test đường dẫn API native engine ổn định.

use crate::native::engine::{
    NativeResolution, resolve_with_protocol, resolve_with_protocol_including_dev,
};

#[test]
fn native_engine_facade_exports_all_public_symbols() {
    fn assert_clone<T: Clone>() {}

    assert_clone::<NativeResolution>();
    let _resolve = resolve_with_protocol;
    let _resolve_with_dev = resolve_with_protocol_including_dev;
}
