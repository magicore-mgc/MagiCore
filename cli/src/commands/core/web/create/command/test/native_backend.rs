use crate::scaffold::spec::ScaffoldRef;

use super::super::native_web_starter_framework;

#[test]
fn native_web_starters_route_supported_default_frameworks_without_version_overrides() {
    assert_eq!(
        native_web_starter_framework(&ScaffoldRef::Default, "backend", Some("axum")),
        Some("axum")
    );
    assert_eq!(
        native_web_starter_framework(&ScaffoldRef::Default, "backend", Some("actix-web")),
        Some("actix-web")
    );
    assert_eq!(
        native_web_starter_framework(&ScaffoldRef::Default, "backend", Some("fastapi")),
        Some("fastapi")
    );
    assert_eq!(
        native_web_starter_framework(&ScaffoldRef::Default, "backend", Some("flask")),
        Some("flask")
    );
    for framework in ["echo", "fiber", "gin"] {
        assert_eq!(
            native_web_starter_framework(&ScaffoldRef::Default, "backend", Some(framework)),
            Some(framework)
        );
    }
    assert_eq!(
        native_web_starter_framework(&ScaffoldRef::Default, "backend", Some("fastify")),
        Some("fastify")
    );
    assert_eq!(
        native_web_starter_framework(&ScaffoldRef::Default, "backend", Some("hono")),
        Some("hono")
    );
    assert_eq!(
        native_web_starter_framework(&ScaffoldRef::Default, "backend", Some("trpc")),
        Some("trpc")
    );
    assert_eq!(
        native_web_starter_framework(&ScaffoldRef::Default, "frontend", Some("astro")),
        Some("astro")
    );
    assert_eq!(
        native_web_starter_framework(
            &ScaffoldRef::Version("1.2.3".to_string()),
            "backend",
            Some("axum")
        ),
        None
    );
    assert_eq!(
        native_web_starter_framework(
            &ScaffoldRef::DistTag("stable".to_string()),
            "backend",
            Some("fastapi")
        ),
        None
    );
    assert_eq!(
        native_web_starter_framework(&ScaffoldRef::Default, "frontend", Some("axum")),
        None
    );
    // django gained a native starter in the CI-ready scaffold pass.
    // django đã có starter native trong đợt scaffold CI-ready.
    assert_eq!(
        native_web_starter_framework(&ScaffoldRef::Default, "backend", Some("django")),
        Some("django")
    );
    assert_eq!(
        native_web_starter_framework(&ScaffoldRef::Default, "backend", Some("laravel")),
        None
    );
}
