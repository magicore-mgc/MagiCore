//! Ordered server data package records.
//! Bản ghi package server data được giữ đúng thứ tự.

use super::super::WebFeaturePackage;

pub(in crate::commands::core::web::implementation::feature_catalog::implementation) const SERVER_DATA: &[WebFeaturePackage] = &[
    WebFeaturePackage {
        feature: "fastify",
        section: "dependencies",
        package: "fastify",
    },
    WebFeaturePackage {
        feature: "nestjs",
        section: "dependencies",
        package: "@nestjs/core",
    },
    WebFeaturePackage {
        feature: "nestjs",
        section: "dependencies",
        package: "@nestjs/common",
    },
    WebFeaturePackage {
        feature: "nestjs",
        section: "dependencies",
        package: "@nestjs/platform-express",
    },
    WebFeaturePackage {
        feature: "hono",
        section: "dependencies",
        package: "hono",
    },
    WebFeaturePackage {
        feature: "koa",
        section: "dependencies",
        package: "koa",
    },
    WebFeaturePackage {
        feature: "koa",
        section: "dependencies",
        package: "@koa/router",
    },
    WebFeaturePackage {
        feature: "typeorm",
        section: "dependencies",
        package: "typeorm",
    },
    WebFeaturePackage {
        feature: "typeorm",
        section: "devDependencies",
        package: "@types/typeorm",
    },
    WebFeaturePackage {
        feature: "mongoose",
        section: "dependencies",
        package: "mongoose",
    },
    WebFeaturePackage {
        feature: "mysql",
        section: "dependencies",
        package: "mysql2",
    },
    WebFeaturePackage {
        feature: "sqlite",
        section: "dependencies",
        package: "better-sqlite3",
    },
    WebFeaturePackage {
        feature: "mongodb",
        section: "dependencies",
        package: "mongodb",
    },
    WebFeaturePackage {
        feature: "yup",
        section: "dependencies",
        package: "yup",
    },
    WebFeaturePackage {
        feature: "joi",
        section: "dependencies",
        package: "joi",
    },
    WebFeaturePackage {
        feature: "valibot",
        section: "dependencies",
        package: "valibot",
    },
    WebFeaturePackage {
        feature: "lucia",
        section: "dependencies",
        package: "lucia",
    },
    WebFeaturePackage {
        feature: "jwt",
        section: "dependencies",
        package: "jsonwebtoken",
    },
    WebFeaturePackage {
        feature: "jwt",
        section: "devDependencies",
        package: "@types/jsonwebtoken",
    },
    WebFeaturePackage {
        feature: "oauth",
        section: "dependencies",
        package: "oauth",
    },
];
