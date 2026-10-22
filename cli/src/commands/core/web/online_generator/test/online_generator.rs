//! Tests for unknown online-generator modes and framework fallback.
//! Test fallback khi mode hoặc framework generator online không được hỗ trợ.

#[test]
fn unsupported_frameworks_and_modes_return_none() {
    assert_eq!(
        super::online_web_generator("frontend", "not-a-framework"),
        None
    );
    assert_eq!(super::online_web_generator("backend", "laravel"), None);
    assert_eq!(super::online_web_generator("fullstack", "react-axum"), None);
}

#[test]
fn integrated_fullstack_uses_official_generator_without_selecting_split_stack() {
    for framework in ["nextjs", "nuxt", "sveltekit", "remix"] {
        let generator = super::online_web_generator("fullstack", framework)
            .expect("integrated framework must have an official initializer");
        assert_eq!(
            Some(generator),
            super::online_web_generator("frontend", framework)
        );
    }
    for framework in ["react-axum", "react-fastify", "react-vite"] {
        assert_eq!(super::online_web_generator("fullstack", framework), None);
    }
}
