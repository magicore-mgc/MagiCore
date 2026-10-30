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
fn required_layer_error_retains_remedy_and_registry_guidance() {
    let error = ScaffoldResolveError::RequiredLayerMissing {
        layer: "web/frontend/nextjs".to_string(),
        core: "web".to_string(),
        template: "nextjs".to_string(),
        tag: "latest".to_string(),
        attempted_sources: "cache miss".to_string(),
    };
    let message = error.to_string();
    assert!(
        message.starts_with(
            "Required scaffold layer 'web/frontend/nextjs' is not available.\n\nTried:\ncache miss\n\nNext steps for 'web' (framework 'nextjs'@latest):\n"
        ),
        "{message}"
    );
    assert!(message.contains("--from-local <path>"), "{message}");
    assert!(message.contains("official initializer"), "{message}");
    assert!(message.contains("MGC_NPM_REGISTRY"), "{message}");
}

#[test]
fn required_layer_error_never_names_a_nonexistent_subcommand() {
    // The previous wording told users to run `mgc template fetch`, and `mgc
    // template` is not a subcommand at all.
    // (Wording cũ bảo user chạy `mgc template fetch`, mà `mgc template` không phải
    // subcommand.)
    let cases = ["web", "game", "iot", "clo", "lib"];
    for core in cases {
        let error = ScaffoldResolveError::RequiredLayerMissing {
            layer: format!("{core}/layer"),
            core: core.to_string(),
            template: "framework".to_string(),
            tag: "latest".to_string(),
            attempted_sources: "HTTP 404".to_string(),
        };
        let message = error.to_string();
        assert!(
            !crate::scaffold::resolver::remedy::names_nonexistent_subcommand(&message),
            "core '{core}' guidance names a nonexistent subcommand: {message}"
        );
    }
}
