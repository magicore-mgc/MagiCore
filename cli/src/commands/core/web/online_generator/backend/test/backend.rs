//! Tests for supported backend initializer records and argument templates.
//! Test record initializer backend được hỗ trợ và mẫu tham số.

#[test]
fn backend_catalog_preserves_supported_initializers_and_flags() {
    let cases = [
        (
            "express",
            "express-generator",
            "express",
            "express-generator:no-view",
        ),
        ("nestjs", "@nestjs/cli", "nest", "nestjs:typescript"),
    ];

    for (framework, package, binary, identity) in cases {
        let generator = super::online_web_generator("backend", framework)
            .unwrap_or_else(|| panic!("missing generator mapping for backend/{framework}"));
        assert_eq!(generator.package, package, "backend/{framework} package");
        assert_eq!(generator.binary, binary, "backend/{framework} binary");
        assert_eq!(generator.identity, identity, "backend/{framework} identity");
        assert!(
            generator
                .args
                .iter()
                .any(|argument| matches!(argument, super::OnlineGeneratorArgument::Project)),
            "backend/{framework} must receive a project name"
        );
    }

    assert_eq!(super::online_web_generator("backend", "fastify"), None);
    assert_eq!(super::online_web_generator("backend", "hono"), None);
}
