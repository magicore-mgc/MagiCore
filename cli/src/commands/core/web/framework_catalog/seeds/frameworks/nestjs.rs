// nestjs framework seed — cấu hình package seed cho nestjs.
// Preserve this framework-specific package set as declared by the MGC catalog.
use super::{WebFrameworkSeed, WebToolchainPackage};

pub(in crate::commands::core::web::implementation::framework_catalog::seeds::implementation) const SEED: WebFrameworkSeed = WebFrameworkSeed {
    name: "nestjs",
    primary: &["@nestjs/core", "@nestjs/common", "@nestjs/platform-express"],
    supplemental: &["reflect-metadata", "rxjs"],
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
            package: "tsx",
            typescript_only: true,
            version: None,
        },
    ],
};
