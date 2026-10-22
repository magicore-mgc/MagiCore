use crate::scaffold::resolver::ScaffoldResolveError;

#[test]
fn only_availability_errors_permit_builtin_fallback() {
    assert!(
        ScaffoldResolveError::RegistryUnavailable {
            registry: "test-registry".to_string(),
            offline: true,
        }
        .is_availability_failure()
    );
    assert!(
        ScaffoldResolveError::RequiredLayerMissing {
            layer: "lib/rust".to_string(),
            core: "lib".to_string(),
            template: "rust".to_string(),
            tag: "latest".to_string(),
            attempted_sources: "no available layer".to_string(),
        }
        .is_availability_failure()
    );
    for error in [
        ScaffoldResolveError::IntegrityMismatch {
            artifact: "template.tgz".to_string(),
            expected: "expected".to_string(),
            actual: "different".to_string(),
        },
        ScaffoldResolveError::Other("linked cache entry refused".to_string()),
        ScaffoldResolveError::InvalidTag {
            template: "rust".to_string(),
            input: "laster".to_string(),
            suggestion: Some("latest".to_string()),
        },
        ScaffoldResolveError::UnsupportedCore {
            core: "unknown".to_string(),
        },
        ScaffoldResolveError::UnsupportedTemplate {
            core: "lib".to_string(),
            template: "unknown".to_string(),
        },
    ] {
        assert!(!error.is_availability_failure(), "{error}");
    }
}

#[test]
fn display_messages_are_stable_and_unsupported_core_lists_all_nine() {
    let cases = [
        (
            ScaffoldResolveError::InvalidTag {
                template: "rust".to_string(),
                input: "laster".to_string(),
                suggestion: Some("latest".to_string()),
            },
            "Unknown scaffold tag 'laster' for 'rust'. Did you mean 'latest'?".to_string(),
        ),
        (
            ScaffoldResolveError::RegistryUnavailable {
                registry: "registry".to_string(),
                offline: true,
            },
            "Registry 'registry' is unavailable (offline mode)".to_string(),
        ),
        (
            ScaffoldResolveError::IntegrityMismatch {
                artifact: "template.tgz".to_string(),
                expected: "abc".to_string(),
                actual: "def".to_string(),
            },
            "Integrity check failed for 'template.tgz': expected abc, got def".to_string(),
        ),
        (
            ScaffoldResolveError::UnsupportedCore {
                core: "unknown".to_string(),
            },
            "Unsupported core: 'unknown'. Supported: web, ai, app, lib, game, iot, cicd, cloud, hardware".to_string(),
        ),
        (
            ScaffoldResolveError::UnsupportedTemplate {
                core: "game".to_string(),
                template: "unknown".to_string(),
            },
            "Unsupported template 'unknown' for core 'game'".to_string(),
        ),
        (
            ScaffoldResolveError::Other("cause".to_string()),
            "Scaffold resolve failed: cause".to_string(),
        ),
    ];
    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn required_layer_error_retains_fetch_and_registry_guidance() {
    let error = ScaffoldResolveError::RequiredLayerMissing {
        layer: "web/frontend/nextjs".to_string(),
        core: "web".to_string(),
        template: "nextjs".to_string(),
        tag: "latest".to_string(),
        attempted_sources: "cache miss".to_string(),
    };
    assert_eq!(
        error.to_string(),
        concat!(
            "Required scaffold layer 'web/frontend/nextjs' is not available.\n\n",
            "Tried:\ncache miss\n\nNext steps:\n",
            "- Run: mgc template fetch web nextjs@latest\n",
            "- Or configure registry: mgc config set registry.scaffold <url>"
        )
    );
}
