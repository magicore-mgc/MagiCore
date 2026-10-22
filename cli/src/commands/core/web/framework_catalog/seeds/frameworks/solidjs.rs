// solidjs framework seed — cấu hình package seed cho solidjs.
// Preserve this framework-specific package set as declared by the MGC catalog.
use super::{WebFrameworkSeed, WebToolchainPackage};

pub(in crate::commands::core::web::implementation::framework_catalog::seeds::implementation) const SEED: WebFrameworkSeed = WebFrameworkSeed {
    name: "solidjs",
    primary: &["solid-js"],
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
            package: "vite-plugin-solid",
            typescript_only: false,
            version: None,
        },
        WebToolchainPackage {
            section: "devDependencies",
            package: "typescript",
            typescript_only: true,
            version: None,
        },
    ],
};
