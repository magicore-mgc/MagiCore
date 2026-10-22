//! Keep canonical names, versions and peer digests stable.
//! Giữ ổn định tên, version canonical và peer digest.

use super::*;

#[test]
fn canonicalizes_ecosystem_names_and_short_versions() {
    assert_eq!(canonical_name(EcosystemTag::Web, " React "), "react");
    assert_eq!(
        canonical_name(EcosystemTag::Python, "Foo_bar.BAZ"),
        "foo-bar-baz"
    );
    assert_eq!(canonical_version("1"), "1.0.0");
    assert_eq!(canonical_version("1.2"), "1.2.0");
    assert_eq!(canonical_version("1.2.3-beta"), "1.2.3-beta");
}

#[test]
fn peer_context_digest_is_order_independent() {
    let first = vec![
        ("react".to_string(), "18.2.0".to_string()),
        ("react-dom".to_string(), "18.2.0".to_string()),
    ];
    let second = vec![first[1].clone(), first[0].clone()];
    assert_eq!(peer_context_digest(&first), peer_context_digest(&second));
}
