// Web command behavior tests — test hành vi command Web.
// Original setups and assertions are retained.

#[test]
fn primary_package_seed_does_not_duplicate_existing_dependency_sections() {
    let package = "@sveltejs/kit";
    let mut manifest = serde_json::Map::new();
    manifest.insert(
        "devDependencies".to_string(),
        serde_json::json!({ package: "^2.0.0" }),
    );
    manifest.insert(
        "dependencies".to_string(),
        serde_json::json!({ "react": "^19.0.0" }),
    );

    super::ensure_primary_package(&mut manifest, package, "^2.69.2");

    let declarations = ["dependencies", "devDependencies"]
        .into_iter()
        .filter(|section| {
            manifest
                .get(*section)
                .and_then(serde_json::Value::as_object)
                .is_some_and(|packages| packages.contains_key(package))
        })
        .count();
    assert_eq!(declarations, 1, "primary package should be declared once");
    assert_eq!(
        manifest["devDependencies"][package], "^2.69.2",
        "the resolved range should update the existing declaration"
    );
    assert!(
        !manifest["dependencies"]
            .as_object()
            .expect("dependencies should remain an object")
            .contains_key(package)
    );
    assert_eq!(manifest["dependencies"]["react"], "^19.0.0");
}

#[test]
fn primary_package_seed_removes_duplicate_from_secondary_section() {
    let package = "@sveltejs/kit";
    let mut manifest = serde_json::json!({
        "dependencies": { package: "^3.0.0" },
        "devDependencies": { package: "^2.0.0" }
    })
    .as_object()
    .expect("manifest should be an object")
    .clone();

    super::ensure_primary_package(&mut manifest, package, "^2.69.2");

    assert_eq!(manifest["dependencies"][package], "^2.69.2");
    assert!(
        !manifest["devDependencies"]
            .as_object()
            .expect("devDependencies should remain an object")
            .contains_key(package)
    );
}

#[test]
fn primary_package_seed_defaults_to_dependencies_when_not_declared() {
    let package = "@sveltejs/kit";
    let mut manifest = serde_json::Map::new();

    super::ensure_primary_package(&mut manifest, package, "^2.69.2");

    assert_eq!(manifest["dependencies"][package], "^2.69.2");
}

#[test]
fn web_preset_only_seeds_implicit_tailwind_without_the_feature_flag() {
    let mut flags = crate::commands::core::scaffold_flags::ScaffoldFlags::default();
    assert!(!super::ui_preset_needs_implicit_tailwind(&flags));

    flags.tailwindcss = true;
    flags.shadcn = true;
    assert!(!super::ui_preset_needs_implicit_tailwind(&flags));

    flags.tailwindcss = false;
    assert!(super::ui_preset_needs_implicit_tailwind(&flags));

    flags.shadcn = false;
    flags.daisyui = true;
    assert!(super::ui_preset_needs_implicit_tailwind(&flags));
}

#[tokio::test]
async fn enrichment_preserves_generator_dependencies_without_registry_resolution() {
    let dir = tempfile::tempdir().unwrap();
    let package = serde_json::json!({
        "name": "hybrid-nest",
        "private": true,
        "dependencies": {
            "@nestjs/core": "^11.0.0",
            "@nestjs/common": "^11.0.0",
            "@nestjs/platform-express": "^11.0.0",
            "reflect-metadata": "^0.2.0",
            "rxjs": "^7.8.0"
        },
        "devDependencies": {
            "typescript": "^6.0.0",
            "@types/node": "^22.0.0",
            "tsx": "^4.0.0",
            "vitest": "^3.0.0"
        },
        "scripts": {"build": "nest build"}
    });
    let manifest = dir.path().join("package.json");
    std::fs::write(&manifest, serde_json::to_vec(&package).unwrap()).unwrap();
    let flags = super::ScaffoldFlags {
        ts: true,
        vitest: true,
        ..Default::default()
    };
    super::enrich_web_project_manifest(
        dir.path(),
        &super::parse_framework_request("nestjs"),
        None,
        &flags,
    )
    .await
    .expect("existing constraints must not require default registry resolution");
    let actual: serde_json::Value =
        serde_json::from_slice(&std::fs::read(manifest).unwrap()).unwrap();
    assert_eq!(
        actual, package,
        "MGC must retain the framework's compatible dependency set"
    );
}
