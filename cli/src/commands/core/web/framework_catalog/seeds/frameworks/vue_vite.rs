// vue-vite framework seed — cấu hình package seed cho vue-vite.
// Preserve this framework-specific package set as declared by the MGC catalog.
use super::{WebFrameworkSeed, WebToolchainPackage};

pub(in crate::commands::core::web::implementation::framework_catalog::seeds::implementation) const SEED: WebFrameworkSeed = WebFrameworkSeed {
    name: "vue-vite",
    primary: &["vue"],
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
            package: "@vitejs/plugin-vue",
            typescript_only: false,
            version: None,
        },
        WebToolchainPackage {
            section: "devDependencies",
            package: "typescript",
            typescript_only: true,
            // Vue Language Tools still needs the TypeScript 6 compiler API;
            // TypeScript 7 no longer exports the ./lib/tsc entry it loads.
            // (Vue Language Tools vẫn cần compiler API TypeScript 6;
            // TypeScript 7 không còn export entry ./lib/tsc mà nó nạp.)
            version: Some("~6.0.3"),
        },
    ],
};
