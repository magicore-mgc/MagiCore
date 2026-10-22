//! Tests for supported frontend initializer records and argument templates.
//! Test record initializer frontend được hỗ trợ và mẫu tham số.

#[test]
fn frontend_catalog_preserves_supported_initializers_and_flags() {
    let cases = [
        ("vanilla", "create-vite", "create-vite", "vite:vanilla-ts"),
        ("react-vite", "create-vite", "create-vite", "vite:react-ts"),
        ("vue-vite", "create-vite", "create-vite", "vite:vue-ts"),
        ("solidjs", "create-vite", "create-vite", "vite:solid-ts"),
        ("qwik", "create-vite", "create-vite", "vite:qwik-ts"),
        (
            "nextjs",
            "create-next-app",
            "create-next-app",
            "create-next-app:default",
        ),
        ("sveltekit", "sv", "sv", "sv:minimal-ts"),
        ("nuxt", "create-nuxt", "create-nuxt", "create-nuxt:default"),
        ("angular", "@angular/cli", "ng", "angular:ng-new-defaults"),
        (
            "remix",
            "create-react-router",
            "create-react-router",
            "create-react-router:default",
        ),
    ];

    for (framework, package, binary, identity) in cases {
        let generator = super::online_web_generator("frontend", framework)
            .unwrap_or_else(|| panic!("missing generator mapping for frontend/{framework}"));
        assert_eq!(generator.package, package, "frontend/{framework} package");
        assert_eq!(generator.binary, binary, "frontend/{framework} binary");
        assert_eq!(
            generator.identity, identity,
            "frontend/{framework} identity"
        );
        assert!(
            generator
                .args
                .iter()
                .any(|argument| matches!(argument, super::OnlineGeneratorArgument::Project)),
            "frontend/{framework} must receive a project name"
        );
    }

    assert_eq!(super::online_web_generator("frontend", "astro"), None);

    let nuxt = super::online_web_generator("frontend", "nuxt").unwrap();
    let nuxt_args = nuxt
        .args
        .iter()
        .filter_map(|argument| match argument {
            super::OnlineGeneratorArgument::Project => None,
            super::OnlineGeneratorArgument::Literal(value) => Some(*value),
        })
        .collect::<Vec<_>>();
    for required in [
        "--template",
        "minimal",
        "--packageManager",
        "npm",
        "--no-gitInit",
        "--no-install",
    ] {
        assert!(
            nuxt_args.contains(&required),
            "Nuxt generator must pass {required}"
        );
    }

    let remix = super::online_web_generator("frontend", "remix").unwrap();
    assert!(
        remix.args.iter().any(|argument| matches!(
            argument,
            super::OnlineGeneratorArgument::Literal("--no-install")
        )),
        "React Router initializer must defer dependency installation to MagiCore"
    );
    assert!(
        remix.args.iter().any(|argument| matches!(
            argument,
            super::OnlineGeneratorArgument::Literal("--no-git-init")
        )),
        "React Router initializer must not initialize Git inside its managed workspace"
    );
}
