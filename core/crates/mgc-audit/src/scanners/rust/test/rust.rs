#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn public_lock_reader_returns_sorted_unique_crates_io_pins() {
    let raw = r#"
version = 4

[[package]]
name = "zeta"
version = "1.0.0"
source = "sparse+https://index.crates.io/"

[[package]]
name = "alpha"
version = "2.0.0"
source = "registry+https://index.crates.io/"

[[package]]
name = "zeta"
version = "1.0.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
"#;

    let (pins, skipped) = read_cargo_lock_pins(raw).unwrap();

    assert!(skipped.is_empty());
    assert_eq!(
        pins.iter()
            .map(|pin| (pin.name.as_str(), pin.version.as_str()))
            .collect::<Vec<_>>(),
        vec![("alpha", "2.0.0"), ("zeta", "1.0.0")]
    );
}
