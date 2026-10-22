//! Ordered platform integrations package records.
//! Bản ghi package platform integrations được giữ đúng thứ tự.

use super::super::WebFeaturePackage;

pub(in crate::commands::core::web::implementation::feature_catalog::implementation) const PLATFORM_INTEGRATIONS: &[WebFeaturePackage] = &[
    WebFeaturePackage {
        feature: "testing-library",
        section: "devDependencies",
        package: "@testing-library/react",
    },
    WebFeaturePackage {
        feature: "testing-library",
        section: "devDependencies",
        package: "@testing-library/jest-dom",
    },
    WebFeaturePackage {
        feature: "turborepo",
        section: "devDependencies",
        package: "turbo",
    },
    WebFeaturePackage {
        feature: "nx",
        section: "devDependencies",
        package: "nx",
    },
    WebFeaturePackage {
        feature: "changesets",
        section: "devDependencies",
        package: "@changesets/cli",
    },
    WebFeaturePackage {
        feature: "grpc",
        section: "dependencies",
        package: "@grpc/grpc-js",
    },
    WebFeaturePackage {
        feature: "grpc",
        section: "dependencies",
        package: "@grpc/proto-loader",
    },
    WebFeaturePackage {
        feature: "pwa",
        section: "devDependencies",
        package: "workbox-webpack-plugin",
    },
    WebFeaturePackage {
        feature: "sentry",
        section: "dependencies",
        package: "@sentry/node",
    },
    WebFeaturePackage {
        feature: "sentry",
        section: "dependencies",
        package: "@sentry/react",
    },
    WebFeaturePackage {
        feature: "analytics",
        section: "dependencies",
        package: "@vercel/analytics",
    },
];
