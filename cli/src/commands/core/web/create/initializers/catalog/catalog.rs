//! Official JavaScript/TypeScript framework initializers.
//! Initializer chính thức của framework JavaScript/TypeScript.
//!
//! Every record names a program published by the framework team. MagiCore
//! never ships the files those programs write, so a framework upgrading its
//! starter needs no MagiCore release.
//! Mỗi record nêu một chương trình do framework team phát hành. MagiCore không
//! bao giờ ship file mà chương trình đó ghi, nên framework nâng cấp starter
//! không cần MagiCore phát hành lại.
//!
//! Frontend frameworks use `--skip-install`/`--no-install` where the
//! initializer offers it: dependencies belong to `mgc install`, which is the
//! only path that writes `mgc.lock`. An initializer that installed its own
//! dependencies with a different package manager would leave a graph MagiCore
//! cannot describe.
//! Framework frontend dùng `--skip-install`/`--no-install` khi initializer có:
//! dependency thuộc về `mgc install`, đường duy nhất ghi `mgc.lock`. Nếu
//! initializer tự cài bằng PM khác thì để lại graph MagiCore không mô tả được.

use super::super::types::{ArgSlot, InitializerRecord, Layout, Runtime, VersionPolicy};

use ArgSlot::{Literal as L, Project as P};

/// Frontend and fullstack records, keyed by normalized framework id.
/// Record frontend và fullstack, khóa theo id framework đã chuẩn hoá.
pub(crate) const FRONTEND: &[(&str, InitializerRecord)] = &[
    (
        "vanilla",
        InitializerRecord {
            package: "create-vite",
            binary: "create-vite",
            identity: "vite:vanilla-ts",
            args: &[P, L("--template"), L("vanilla-ts")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "react",
        InitializerRecord {
            package: "create-vite",
            binary: "create-vite",
            identity: "vite:react-ts",
            args: &[P, L("--template"), L("react-ts")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "react-vite",
        InitializerRecord {
            package: "create-vite",
            binary: "create-vite",
            identity: "vite:react-ts",
            args: &[P, L("--template"), L("react-ts")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "vue",
        InitializerRecord {
            package: "create-vite",
            binary: "create-vite",
            identity: "vite:vue-ts",
            args: &[P, L("--template"), L("vue-ts")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "vue-vite",
        InitializerRecord {
            package: "create-vite",
            binary: "create-vite",
            identity: "vite:vue-ts",
            args: &[P, L("--template"), L("vue-ts")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "solid",
        InitializerRecord {
            package: "create-vite",
            binary: "create-vite",
            identity: "vite:solid-ts",
            args: &[P, L("--template"), L("solid-ts")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "solidjs",
        InitializerRecord {
            package: "create-vite",
            binary: "create-vite",
            identity: "vite:solid-ts",
            args: &[P, L("--template"), L("solid-ts")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "qwik",
        InitializerRecord {
            package: "create-vite",
            binary: "create-vite",
            identity: "vite:qwik-ts",
            args: &[P, L("--template"), L("qwik-ts")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "svelte",
        InitializerRecord {
            package: "sv",
            binary: "sv",
            identity: "sv:minimal-ts",
            args: &[
                L("create"),
                L("--template"),
                L("minimal"),
                L("--types"),
                L("ts"),
                L("--no-add-ons"),
                L("--no-install"),
                P,
            ],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "sveltekit",
        InitializerRecord {
            package: "sv",
            binary: "sv",
            identity: "sv:minimal-ts",
            args: &[
                L("create"),
                L("--template"),
                L("minimal"),
                L("--types"),
                L("ts"),
                L("--no-add-ons"),
                L("--no-install"),
                P,
            ],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "nextjs",
        InitializerRecord {
            package: "create-next-app",
            binary: "create-next-app",
            identity: "create-next-app:default",
            args: &[P, L("--yes"), L("--skip-install"), L("--disable-git")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "next",
        InitializerRecord {
            package: "create-next-app",
            binary: "create-next-app",
            identity: "create-next-app:default",
            args: &[P, L("--yes"), L("--skip-install"), L("--disable-git")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "nuxt",
        InitializerRecord {
            package: "create-nuxt",
            binary: "create-nuxt",
            identity: "create-nuxt:default",
            args: &[
                P,
                L("--template"),
                L("minimal"),
                L("--packageManager"),
                L("npm"),
                L("--no-gitInit"),
                L("--no-install"),
                L("--no-modules"),
            ],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "angular",
        InitializerRecord {
            package: "@angular/cli",
            binary: "ng",
            identity: "angular:ng-new-defaults",
            args: &[
                L("new"),
                P,
                L("--defaults"),
                L("--skip-install"),
                L("--skip-git"),
                L("--interactive=false"),
                L("--package-manager"),
                L("npm"),
            ],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "remix",
        InitializerRecord {
            package: "create-react-router",
            binary: "create-react-router",
            identity: "create-react-router:default",
            args: &[P, L("--no-install"), L("--no-git-init")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "astro",
        InitializerRecord {
            package: "create-astro",
            binary: "create-astro",
            identity: "create-astro:minimal",
            args: &[
                P,
                L("--template"),
                L("minimal"),
                L("--typescript"),
                L("strict"),
                L("--no-install"),
                L("--no-git"),
                L("--skip-houston"),
            ],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "vite",
        InitializerRecord {
            package: "create-vite",
            binary: "create-vite",
            identity: "vite:vanilla-ts",
            args: &[P, L("--template"), L("vanilla-ts")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "preact",
        InitializerRecord {
            package: "create-vite",
            binary: "create-vite",
            identity: "vite:preact-ts",
            args: &[P, L("--template"), L("preact-ts")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "lit",
        InitializerRecord {
            package: "create-vite",
            binary: "create-vite",
            identity: "vite:lit-ts",
            args: &[P, L("--template"), L("lit-ts")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "vue-cli",
        InitializerRecord {
            package: "create-vue",
            binary: "create-vue",
            identity: "create-vue:ts",
            args: &[P, L("--ts"), L("--default"), L("--bare")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "docusaurus",
        InitializerRecord {
            package: "create-docusaurus",
            binary: "create-docusaurus",
            identity: "create-docusaurus:classic-ts",
            args: &[P, L("classic"), L("--typescript"), L("--skip-install")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "vitepress",
        InitializerRecord {
            package: "create-vitepress",
            binary: "create-vitepress",
            identity: "create-vitepress:default",
            args: &[P],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "turbo",
        InitializerRecord {
            package: "create-turbo",
            binary: "create-turbo",
            identity: "create-turbo:default",
            args: &[P, L("--skip-install"), L("--package-manager"), L("npm")],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "tauri",
        InitializerRecord {
            package: "create-tauri-app",
            binary: "create-tauri-app",
            identity: "create-tauri-app:react-ts",
            args: &[
                P,
                L("--manager"),
                L("npm"),
                L("--template"),
                L("react-ts"),
                L("--yes"),
            ],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
];

/// Backend records, keyed by normalized framework id.
/// Record backend, khóa theo id framework đã chuẩn hoá.
pub(crate) const BACKEND: &[(&str, InitializerRecord)] = &[
    (
        "spring-boot",
        InitializerRecord {
            package: "start.spring.io",
            binary: "starter.zip",
            identity: "start.spring.io:maven-java",
            args: &[],
            layout: Layout::InPlace,
            version: VersionPolicy::Latest,
            runtime: Runtime::HttpArchive {
                endpoint: "https://start.spring.io/starter.zip",
                params: &[
                    ("type", "maven-project"),
                    ("language", "java"),
                    ("packaging", "jar"),
                    ("javaVersion", "21"),
                    ("dependencies", "web"),
                    ("baseDir", ""),
                ],
                name_param: "artifactId",
                unwrap_single_root: false,
            },
        },
    ),
    (
        "quarkus",
        InitializerRecord {
            package: "code.quarkus.io",
            binary: "project.zip",
            identity: "code.quarkus.io:maven-java",
            args: &[],
            layout: Layout::InPlace,
            version: VersionPolicy::Latest,
            runtime: Runtime::HttpArchive {
                endpoint: "https://code.quarkus.io/d",
                params: &[
                    ("b", "MAVEN"),
                    ("j", "21"),
                    ("e", "rest"),
                    ("cn", "Quarkus Application"),
                ],
                name_param: "cn",
                unwrap_single_root: true,
            },
        },
    ),
    (
        "express",
        InitializerRecord {
            package: "express-generator",
            binary: "express",
            identity: "express-generator:no-view",
            args: &[L("--no-view"), P],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "nestjs",
        InitializerRecord {
            package: "@nestjs/cli",
            binary: "nest",
            identity: "nestjs:typescript",
            args: &[
                L("new"),
                P,
                L("--skip-install"),
                L("--skip-git"),
                L("--package-manager"),
                L("npm"),
                L("--language"),
                L("TS"),
                L("--no-observe"),
            ],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
    (
        "nest",
        InitializerRecord {
            package: "@nestjs/cli",
            binary: "nest",
            identity: "nestjs:typescript",
            args: &[
                L("new"),
                P,
                L("--skip-install"),
                L("--skip-git"),
                L("--package-manager"),
                L("npm"),
                L("--language"),
                L("TS"),
                L("--no-observe"),
            ],
            layout: Layout::NestedProject,
            version: VersionPolicy::Latest,
            runtime: Runtime::RegistryPackage,
        },
    ),
];
