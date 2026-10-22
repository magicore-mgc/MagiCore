//! Static catalog of official frontend initializer packages and arguments.
//! Catalog tĩnh package initializer frontend và tham số chính thức.

use super::super::types::{OnlineGeneratorArgument, OnlineWebGenerator};

/// Return one frontend initializer record without launching a package.
/// Trả record initializer frontend, không tự chạy package.
pub(in crate::commands::core::web::implementation::online_generator) fn frontend_web_generator(
    framework: &str,
) -> Option<OnlineWebGenerator> {
    use OnlineGeneratorArgument::{Literal as L, Project as P};

    let (package, binary, identity, args) = match framework {
        "vanilla" => (
            "create-vite",
            "create-vite",
            "vite:vanilla-ts",
            &[P, L("--template"), L("vanilla-ts")][..],
        ),
        "react-vite" => (
            "create-vite",
            "create-vite",
            "vite:react-ts",
            &[P, L("--template"), L("react-ts")][..],
        ),
        "vue-vite" => (
            "create-vite",
            "create-vite",
            "vite:vue-ts",
            &[P, L("--template"), L("vue-ts")][..],
        ),
        "solidjs" => (
            "create-vite",
            "create-vite",
            "vite:solid-ts",
            &[P, L("--template"), L("solid-ts")][..],
        ),
        "qwik" => (
            "create-vite",
            "create-vite",
            "vite:qwik-ts",
            &[P, L("--template"), L("qwik-ts")][..],
        ),
        "nextjs" => (
            "create-next-app",
            "create-next-app",
            "create-next-app:default",
            &[P, L("--yes"), L("--skip-install"), L("--disable-git")][..],
        ),
        "sveltekit" => (
            "sv",
            "sv",
            "sv:minimal-ts",
            &[
                L("create"),
                L("--template"),
                L("minimal"),
                L("--types"),
                L("ts"),
                L("--no-add-ons"),
                L("--no-install"),
                P,
            ][..],
        ),
        "nuxt" => (
            "create-nuxt",
            "create-nuxt",
            "create-nuxt:default",
            &[
                P,
                L("--template"),
                L("minimal"),
                L("--packageManager"),
                L("npm"),
                L("--no-gitInit"),
                L("--no-install"),
                L("--no-modules"),
            ][..],
        ),
        "angular" => (
            "@angular/cli",
            "ng",
            "angular:ng-new-defaults",
            &[
                L("new"),
                P,
                L("--defaults"),
                L("--skip-install"),
                L("--skip-git"),
                L("--interactive=false"),
                L("--package-manager"),
                L("npm"),
            ][..],
        ),
        "remix" => (
            "create-react-router",
            "create-react-router",
            "create-react-router:default",
            &[P, L("--no-install"), L("--no-git-init")][..],
        ),
        _ => return None,
    };

    Some(OnlineWebGenerator {
        package,
        binary,
        identity,
        args,
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
