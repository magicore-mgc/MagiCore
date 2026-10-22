//! Add Options nearest-owner contract tests.
//! Test contract gần owner add options.

use crate::adapter::AddOptions;

#[test]
fn add_options_default() {
    let opts = AddOptions::default();
    assert!(!opts.dev);
    assert!(!opts.optional);
    assert!(!opts.peer);
    assert!(!opts.exact);
}
