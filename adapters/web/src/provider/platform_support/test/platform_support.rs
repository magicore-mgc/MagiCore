//! Host native-package selection / chọn package native cho host.

use crate::provider::NpmDependencyProvider;
use mgc_types::PackageName;

#[test]
fn test_known_optional_native_binary_supported_only_matches_current_target() {
    let supported = PackageName::new(format!(
        "@esbuild/{}-{}",
        NpmDependencyProvider::current_npm_os(),
        NpmDependencyProvider::current_npm_cpu()
    ))
    .unwrap();
    let unsupported = PackageName::new("@esbuild/linux-s390x").unwrap();
    let codex_supported = PackageName::new(format!(
        "@openai/codex-{}-{}",
        NpmDependencyProvider::current_npm_os(),
        NpmDependencyProvider::current_npm_cpu()
    ))
    .unwrap();
    let codex_unsupported = PackageName::new("@openai/codex-darwin-arm64").unwrap();
    let typescript_supported = PackageName::new(format!(
        "@typescript/typescript-{}-{}",
        NpmDependencyProvider::current_npm_os(),
        NpmDependencyProvider::current_npm_cpu()
    ))
    .unwrap();
    let typescript_unsupported_target = if format!(
        "{}-{}",
        NpmDependencyProvider::current_npm_os(),
        NpmDependencyProvider::current_npm_cpu()
    ) == "linux-x64"
    {
        "darwin-arm64"
    } else {
        "linux-x64"
    };
    let typescript_unsupported = PackageName::new(format!(
        "@typescript/typescript-{typescript_unsupported_target}"
    ))
    .unwrap();
    let unknown = PackageName::new("optional-but-not-native").unwrap();

    assert_eq!(
        NpmDependencyProvider::known_optional_native_binary_supported(&supported),
        Some(true)
    );
    assert_eq!(
        NpmDependencyProvider::known_optional_native_binary_supported(&unsupported),
        Some(false)
    );
    assert_eq!(
        NpmDependencyProvider::known_optional_native_binary_supported(&codex_supported),
        Some(true)
    );
    if format!(
        "{}-{}",
        NpmDependencyProvider::current_npm_os(),
        NpmDependencyProvider::current_npm_cpu()
    ) != "darwin-arm64"
    {
        assert_eq!(
            NpmDependencyProvider::known_optional_native_binary_supported(&codex_unsupported),
            Some(false)
        );
    }
    assert_eq!(
        NpmDependencyProvider::known_optional_native_binary_supported(&typescript_supported),
        Some(true)
    );
    assert_eq!(
        NpmDependencyProvider::known_optional_native_binary_supported(&typescript_unsupported),
        Some(false)
    );
    assert_eq!(
        NpmDependencyProvider::known_optional_native_binary_supported(&unknown),
        None
    );
}
