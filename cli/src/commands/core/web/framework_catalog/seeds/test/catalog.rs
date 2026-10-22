// Framework seed catalog contracts — hợp đồng dữ liệu catalog framework.
// Keep framework constraints and the ordered catalog close to their data owner.
use super::super::FRAMEWORK_SEEDS;

#[test]
fn framework_seed_catalog_keeps_stable_order() {
    let actual: Vec<_> = FRAMEWORK_SEEDS.iter().map(|seed| seed.name).collect();
    assert_eq!(
        actual,
        vec![
            "vanilla",
            "solidjs",
            "sveltekit",
            "react-vite",
            "nextjs",
            "fastify",
            "vue-vite",
            "nuxt",
            "angular",
            "qwik",
            "astro",
            "express",
            "hono",
            "nestjs",
            "trpc",
        ]
    );
}

#[test]
fn nuxt_scaffold_seeds_unplugin_at_project_root_for_builder_resolution() {
    let seed = super::FRAMEWORK_SEEDS
        .iter()
        .find(|seed| seed.name == "nuxt")
        .expect("Nuxt framework seed should exist");

    assert!(seed.primary.contains(&"nuxt"));
    assert!(seed.supplemental.contains(&"vue"));
    assert!(seed.toolchain.iter().any(|package| {
        package.section == "devDependencies"
            && package.package == "unplugin"
            && package.version.is_none()
    }));
}

#[test]
fn angular_scaffold_keeps_typescript_inside_the_compiler_supported_minor() {
    let seed = super::FRAMEWORK_SEEDS
        .iter()
        .find(|seed| seed.name == "angular")
        .expect("Angular framework seed should exist");

    let typescript = seed
        .toolchain
        .iter()
        .find(|package| package.package == "typescript")
        .expect("Angular must declare its TypeScript compiler peer");

    assert_eq!(typescript.version, Some("~6.0.3"));
}

#[test]
fn vue_vite_scaffold_keeps_typescript_inside_the_language_tools_api_range() {
    let seed = super::FRAMEWORK_SEEDS
        .iter()
        .find(|seed| seed.name == "vue-vite")
        .expect("Vue Vite framework seed should exist");

    let typescript = seed
        .toolchain
        .iter()
        .find(|package| package.package == "typescript")
        .expect("Vue Vite must declare its TypeScript compiler peer");

    assert_eq!(typescript.version, Some("~6.0.3"));
}
