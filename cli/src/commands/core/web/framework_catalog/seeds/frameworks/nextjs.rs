// nextjs framework seed — cấu hình package seed cho nextjs.
// Preserve this framework-specific package set as declared by the MGC catalog.
use super::{WebFrameworkSeed, WebToolchainPackage};

pub(in crate::commands::core::web::implementation::framework_catalog::seeds::implementation) const SEED: WebFrameworkSeed = WebFrameworkSeed {
    name: "nextjs",
    primary: &["next"],
    supplemental: &["react", "react-dom"],
    toolchain: &[
        WebToolchainPackage {
            section: "devDependencies",
            package: "typescript",
            typescript_only: true,
            version: None,
        },
        WebToolchainPackage {
            section: "devDependencies",
            package: "@types/node",
            typescript_only: true,
            version: None,
        },
        WebToolchainPackage {
            section: "devDependencies",
            package: "@types/react",
            typescript_only: true,
            version: None,
        },
        WebToolchainPackage {
            section: "devDependencies",
            package: "@types/react-dom",
            typescript_only: true,
            version: None,
        },
    ],
};
