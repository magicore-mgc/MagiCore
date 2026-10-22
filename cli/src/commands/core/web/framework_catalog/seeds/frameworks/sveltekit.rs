// sveltekit framework seed — cấu hình package seed cho sveltekit.
// Preserve this framework-specific package set as declared by the MGC catalog.
use super::{WebFrameworkSeed, WebToolchainPackage};

pub(in crate::commands::core::web::implementation::framework_catalog::seeds::implementation) const SEED: WebFrameworkSeed = WebFrameworkSeed {
    name: "sveltekit",
    primary: &["@sveltejs/kit"],
    supplemental: &[],
    toolchain: &[
        WebToolchainPackage {
            section: "devDependencies",
            package: "vite",
            typescript_only: false,
            version: None,
        },
        WebToolchainPackage {
            section: "devDependencies",
            package: "@sveltejs/vite-plugin-svelte",
            typescript_only: false,
            version: None,
        },
        WebToolchainPackage {
            section: "devDependencies",
            package: "@sveltejs/adapter-auto",
            typescript_only: false,
            version: None,
        },
        WebToolchainPackage {
            section: "devDependencies",
            package: "svelte",
            typescript_only: false,
            version: None,
        },
        WebToolchainPackage {
            section: "devDependencies",
            package: "typescript",
            typescript_only: true,
            version: Some("^6.0.3"),
        },
    ],
};
