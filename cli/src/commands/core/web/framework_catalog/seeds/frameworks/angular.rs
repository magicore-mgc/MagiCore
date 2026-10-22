// angular framework seed — cấu hình package seed cho angular.
// Preserve this framework-specific package set as declared by the MGC catalog.
use super::{WebFrameworkSeed, WebToolchainPackage};

pub(in crate::commands::core::web::implementation::framework_catalog::seeds::implementation) const SEED: WebFrameworkSeed = WebFrameworkSeed {
    name: "angular",
    primary: &[
        "@angular/core",
        "@angular/compiler",
        "@angular/common",
        "@angular/platform-browser",
        "@angular/platform-browser-dynamic",
        "@angular/router",
    ],
    supplemental: &["rxjs", "zone.js", "tslib"],
    toolchain: &[
        WebToolchainPackage {
            section: "devDependencies",
            package: "@angular/cli",
            typescript_only: false,
            version: None,
        },
        WebToolchainPackage {
            section: "devDependencies",
            package: "@angular/compiler-cli",
            typescript_only: true,
            version: None,
        },
        WebToolchainPackage {
            section: "devDependencies",
            package: "@angular-devkit/build-angular",
            typescript_only: false,
            version: None,
        },
        WebToolchainPackage {
            section: "devDependencies",
            package: "typescript",
            typescript_only: true,
            // Angular 22.2 supports TypeScript >=6.0.0 <6.1.0; the
            // global latest major (TypeScript 7) breaks compiler-cli.
            // (Angular 22.2 cần TypeScript >=6.0.0 <6.1.0; major mới
            // nhất TypeScript 7 làm lỗi compiler-cli.)
            version: Some("~6.0.3"),
        },
    ],
};
