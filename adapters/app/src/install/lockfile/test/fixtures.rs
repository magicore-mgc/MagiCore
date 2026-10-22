//! Test-only App lock fixtures / Fixture lock App chỉ dùng trong test.

use mgc_lockfile::{EcosystemTag, Package};
use std::path::Path;

pub(crate) fn package(name: &str, version: &str, ecosystem: EcosystemTag) -> Package {
    Package {
        name: name.to_string(),
        version: version.to_string(),
        ecosystem,
        registry: (ecosystem != EcosystemTag::Other).then(|| format!("test://{ecosystem}")),
        ..Default::default()
    }
}

pub(crate) fn v4_package(name: &str, version: &str) -> Package {
    Package {
        name: name.to_string(),
        version: version.to_string(),
        ecosystem: EcosystemTag::Dart,
        resolved: format!("https://pub.dev/api/archives/{name}-{version}.tar.gz"),
        integrity: "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=".to_string(),
        registry: Some("https://pub.dev".to_string()),
        ..Default::default()
    }
}

pub(crate) fn write_mgc_toml_schema(dir: &Path, schema: &str) {
    std::fs::write(
        dir.join("mgc.toml"),
        format!("name = \"app\"\necosystem = \"app\"\n[lock]\nschema = \"{schema}\"\n"),
    )
    .unwrap();
}
