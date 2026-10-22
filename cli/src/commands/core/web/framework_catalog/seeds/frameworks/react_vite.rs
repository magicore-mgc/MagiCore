// react-vite framework seed — cấu hình package seed cho react-vite.
// Preserve this framework-specific package set as declared by the MGC catalog.
use super::{WebFrameworkSeed, WebToolchainPackage};

pub(in crate::commands::core::web::implementation::framework_catalog::seeds::implementation) const SEED: WebFrameworkSeed = WebFrameworkSeed {
    name: "react-vite",
    primary: &["react", "react-dom"],
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
            package: "@vitejs/plugin-react",
            typescript_only: false,
            version: None,
        },
        WebToolchainPackage {
            section: "devDependencies",
            package: "typescript",
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
