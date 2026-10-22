// Resolve package versions and update generated manifests — chọn phiên bản và cập nhật manifest sinh ra.
// Web manifest package seeding.
use super::*;

pub(in crate::commands::core::web::implementation) async fn resolve_primary_version(
    request: &FrameworkRequest,
    package: &str,
    age_policy: Option<versions::ScaffoldAgePolicy>,
) -> Result<String> {
    if let Some(tag) = request.dist_tag.as_deref() {
        return versions::resolve_npm_dist_tag_version(package, tag, age_policy).await;
    }
    match request.version.as_deref() {
        Some("latest") | None => versions::fetch_npm_latest_version(package, age_policy).await,
        Some(version) => {
            versions::resolve_seed_dependency_version(package, Some(version), age_policy).await
        }
    }
}
pub(super) fn ensure_package(
    root: &mut Map<String, Value>,
    section: &str,
    package: &str,
    version: &str,
) {
    let entry = root
        .entry(section.to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    if let Value::Object(map) = entry {
        map.insert(package.to_string(), Value::String(version.to_string()));
    }
}
/// Update the existing primary-package section and remove duplicate declarations.
/// Cập nhật section hiện có của primary package và xóa khai báo trùng.
pub(in crate::commands::core::web::implementation) fn ensure_primary_package(
    root: &mut Map<String, Value>,
    package: &str,
    version: &str,
) {
    const DEPENDENCY_SECTIONS: [&str; 2] = ["dependencies", "devDependencies"];
    let existing_section = DEPENDENCY_SECTIONS.iter().copied().find(|section| {
        root.get(*section)
            .and_then(Value::as_object)
            .is_some_and(|packages| packages.contains_key(package))
    });
    let target_section = existing_section.unwrap_or("dependencies");

    for section in DEPENDENCY_SECTIONS {
        if let Some(packages) = root.get_mut(section).and_then(Value::as_object_mut) {
            if section == target_section {
                packages.insert(package.to_string(), Value::String(version.to_string()));
            } else {
                packages.remove(package);
            }
        }
    }

    if existing_section.is_none() {
        ensure_package(root, "dependencies", package, version);
    }
}
/// Preserve declared generator/user constraints before resolving defaults.
/// Giữ constraint của generator/user thay vì tự nâng toolchain ngoài ý muốn.
pub(super) fn has_declared_package(root: &Map<String, Value>, package: &str) -> bool {
    [
        "dependencies",
        "devDependencies",
        "optionalDependencies",
        "peerDependencies",
    ]
    .iter()
    .any(|section| {
        root.get(*section)
            .and_then(Value::as_object)
            .and_then(|packages| packages.get(package))
            .and_then(Value::as_str)
            .is_some_and(|version| !version.is_empty())
    })
}

async fn apply_web_manifest_seed(
    package_json_path: &Path,
    request: &FrameworkRequest,
    flags: &ScaffoldFlags,
) -> Result<()> {
    if !package_json_path.exists() {
        return Ok(());
    }
    let mut value: Value = serde_json::from_str(&std::fs::read_to_string(package_json_path)?)?;
    let object = value
        .as_object_mut()
        .ok_or_else(crate::error::package_json_root_object)?;
    let project_root = package_json_path.parent().unwrap_or_else(|| Path::new("."));
    let age_policy = versions::scaffold_age_policy(project_root)?;

    let seed_name = resolve_seed_name(&request.normalized);
    if let Some(seed) = FRAMEWORK_SEEDS.iter().find(|s| s.name == seed_name) {
        // Resolve paired primaries independently; registry publications can be staggered.
        // Giải quyết từng package primary riêng vì registry có thể phát hành lệch thời điểm.
        for &package in seed.primary {
            if request.version.is_none()
                && request.dist_tag.is_none()
                && has_declared_package(object, package)
            {
                continue;
            }
            let primary = resolve_primary_version(request, package, age_policy).await?;
            ensure_primary_package(object, package, &primary);
        }
        for &package in seed.supplemental {
            if has_declared_package(object, package) {
                continue;
            }
            let version =
                versions::resolve_seed_dependency_version(package, None, age_policy).await?;
            ensure_package(object, "dependencies", package, &version);
        }
        for tool in seed.toolchain {
            if (tool.typescript_only && !flags.ts) || has_declared_package(object, tool.package) {
                continue;
            }
            let version =
                versions::resolve_seed_dependency_version(tool.package, tool.version, age_policy)
                    .await?;
            ensure_package(object, tool.section, tool.package, &version);
        }
    }

    if ui_preset_needs_implicit_tailwind(flags) && !has_declared_package(object, "tailwindcss") {
        let version =
            versions::resolve_seed_dependency_version("tailwindcss", None, age_policy).await?;
        ensure_package(object, "devDependencies", "tailwindcss", &version);
    }

    for feat_pkg in FEATURE_PACKAGE_GROUPS.iter().flat_map(|group| group.iter()) {
        let is_active = match feat_pkg.feature {
            "prisma" => flags.prisma,
            "vitest" => flags.vitest,
            "eslint" => flags.eslint,
            "prettier" => flags.prettier,
            "tailwindcss" => flags.tailwindcss,
            "postgres" => flags.postgres,
            "zustand" => flags.zustand,
            "tanstack-query" => flags.tanstack_query,
            "zod" => flags.zod,
            "shadcn" => flags.shadcn,
            "nextauth" => flags.nextauth,
            "playwright" => flags.playwright,
            "husky" => flags.husky,
            "lint-staged" => flags.lint_staged,
            "biome" => flags.biome,
            "sass" => flags.sass,
            "unocss" => flags.unocss,
            "daisyui" => flags.daisyui,
            "redux" => flags.redux,
            "jest" => flags.jest,
            "cypress" => flags.cypress,
            "drizzle" => flags.drizzle,
            "clerk" => flags.clerk,
            "styled-components" => flags.styled_components,
            "commitlint" => flags.commitlint,
            "rest" => flags.rest,
            "graphql" => flags.graphql,
            "trpc" => flags.trpc,
            "grpc" => flags.grpc,
            "lucia" => flags.lucia,
            "jwt" => flags.jwt,
            "oauth" => flags.oauth,
            "dotenv" => flags.dotenv,
            "i18n" => flags.i18n,
            "pwa" => flags.pwa,
            "storybook" => flags.storybook,
            "sentry" => flags.sentry,
            "analytics" => flags.analytics,
            "railway" => flags.railway,
            "fly" => flags.fly,
            _ => false,
        };
        if is_active && !has_declared_package(object, feat_pkg.package) {
            let version =
                versions::resolve_seed_dependency_version(feat_pkg.package, None, age_policy)
                    .await?;
            ensure_package(object, feat_pkg.section, feat_pkg.package, &version);
        }
    }

    // P0 finding (2026-09-12): `mgc test` fails honestly ("No test
    // runner detected") on scaffolds that ship no test script — every
    // fullstack/vite template that carries TypeScript must ship a
    // `test` script (tsc --noEmit typecheck is the honest baseline
    // smoke; vitest/jest flags override it via ensure_script below).
    // This is the scaffold's OWN test — no CI printf-ing tests into
    // user projects at run time.
    // P0 finding (2026-09-12): `mgc test` fail trung thực ("No test
    // runner detected") trên template không có script test — mọi
    // template vite có TypeScript phải TỰ mang script `test` (tsc
    // --noEmit là smoke baseline trung thực; flag vitest/jest ghi đè
    // qua ensure_script ở dưới). Đây là test CỦA scaffold — CI không
    // bịa test vào project user lúc chạy.
    if flags.ts
        && matches!(
            seed_name,
            "react-vite" | "vue-vite" | "solid" | "svelte" | "qwik"
        )
    {
        ensure_script(object, "test", "tsc --noEmit");
    }

    std::fs::write(package_json_path, serde_json::to_string_pretty(&value)?)?;
    Ok(())
}
/// Insert a scripts entry only when the scaffold did not define it —
/// user/template choices win over the default baseline.
/// Chèn mục scripts chỉ khi scaffold chưa định nghĩa — lựa chọn của
/// user/template thắng baseline mặc định.
pub(super) fn ensure_script(root: &mut Map<String, Value>, name: &str, command: &str) {
    let scripts = root
        .entry("scripts".to_string())
        .or_insert_with(|| Value::Object(Map::new()));
    if let Value::Object(map) = scripts {
        map.entry(name.to_string())
            .or_insert_with(|| Value::String(command.to_string()));
    }
}
pub(crate) async fn enrich_web_project_manifest(
    project_dir: &Path,
    frontend: &FrameworkRequest,
    backend: Option<&FrameworkRequest>,
    flags: &ScaffoldFlags,
) -> Result<()> {
    if flags.monorepo {
        apply_web_manifest_seed(
            &project_dir
                .join("apps")
                .join("frontend")
                .join("package.json"),
            frontend,
            flags,
        )
        .await?;
        if let Some(backend) = backend {
            apply_web_manifest_seed(
                &project_dir
                    .join("apps")
                    .join("backend")
                    .join("package.json"),
                backend,
                flags,
            )
            .await?;
        }
        return Ok(());
    }
    if let Some(backend) = backend {
        apply_web_manifest_seed(&project_dir.join("package.json"), frontend, flags).await?;
        apply_web_manifest_seed(
            &project_dir.join("server").join("package.json"),
            backend,
            flags,
        )
        .await
    } else {
        apply_web_manifest_seed(&project_dir.join("package.json"), frontend, flags).await
    }
}
