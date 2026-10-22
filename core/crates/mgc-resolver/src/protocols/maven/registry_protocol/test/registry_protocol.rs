//! Maven RegistryProtocol trait behavior test.
//! Test hành vi trait RegistryProtocol Maven.

use crate::protocols::MavenProtocol;
use crate::protocols::RegistryProtocol;
use mgc_types::Version;

#[test]
fn trait_implementation_uses_maven_range_matching() {
    let protocol = MavenProtocol::new("https://repo.example.test/maven2");
    let version = Version::parse("1.2.3").expect("test operation must succeed");
    assert!(protocol.matches_requirement("[1.2.3]", &version));
}
