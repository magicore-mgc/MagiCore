// nuxt framework seed — cấu hình package seed cho nuxt.
// Preserve this framework-specific package set as declared by the MGC catalog.
use super::{WebFrameworkSeed, WebToolchainPackage};

pub(in crate::commands::core::web::implementation::framework_catalog::seeds::implementation) const SEED: WebFrameworkSeed = WebFrameworkSeed {
    name: "nuxt",
    primary: &["nuxt"],
    supplemental: &["vue"],
    toolchain: &[
        WebToolchainPackage {
            section: "devDependencies",
            package: "unplugin",
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
