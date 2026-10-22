#![allow(clippy::unwrap_used)]

use super::super::WEB_FRAMEWORKS;
use std::collections::HashSet;

#[test]
fn catalog_names_are_unique_and_metadata_matches_framework_kind() {
    let mut names = HashSet::new();
    assert!(!WEB_FRAMEWORKS.is_empty());
    let ordered_names = WEB_FRAMEWORKS
        .iter()
        .map(|framework| framework.name)
        .collect::<Vec<_>>();
    assert_eq!(
        ordered_names,
        vec![
            "vanilla",
            "react-vite",
            "vue-vite",
            "nextjs",
            "sveltekit",
            "nuxt",
            "angular",
            "solidjs",
            "qwik",
            "astro",
            "express",
            "fastify",
            "nestjs",
            "hono",
            "trpc",
            "laravel",
            "symfony",
            "spring-boot",
            "quarkus",
            "dotnet-minimal",
            "dotnet-webapi",
            "gin",
            "echo",
            "fiber",
            "fastapi",
            "django",
            "flask",
            "axum",
            "actix-web",
            "remix",
            "react-fastify",
            "vue-laravel",
            "react-spring",
            "custom",
            "react-express",
            "react-hono",
            "react-nestjs",
            "react-trpc",
            "vue-express",
            "vue-hono",
            "vue-nestjs",
            "svelte-express",
            "svelte-hono",
            "react-axum",
            "react-actix-web",
            "react-gin",
            "react-echo",
            "react-fiber",
            "react-fastapi",
            "react-django",
            "react-flask",
            "react-quarkus",
            "react-symfony",
            "vue-axum",
            "vue-actix-web",
            "vue-gin",
            "vue-echo",
            "vue-fiber",
            "vue-fastapi",
            "vue-django",
            "vue-flask",
            "vue-quarkus",
            "vue-symfony",
            "svelte-axum",
            "svelte-gin",
            "svelte-fastapi",
            "svelte-quarkus",
        ]
    );

    for framework in WEB_FRAMEWORKS {
        assert!(!framework.name.is_empty());
        assert!(
            names.insert(framework.name),
            "duplicate framework: {}",
            framework.name
        );
        match framework.sub_type {
            "frontend" | "fullstack" => assert!(
                framework.base.is_none(),
                "{} has an unexpected backend base",
                framework.name
            ),
            "backend" => assert!(
                framework.base.is_some_and(|base| !base.is_empty()),
                "{} is missing its language base",
                framework.name
            ),
            other => panic!("unknown framework subtype: {other}"),
        }
    }
}
