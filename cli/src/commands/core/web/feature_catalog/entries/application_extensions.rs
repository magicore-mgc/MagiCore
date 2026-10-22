//! Ordered application extensions package records.
//! Bản ghi package application extensions được giữ đúng thứ tự.

use super::super::WebFeaturePackage;

pub(in crate::commands::core::web::implementation::feature_catalog::implementation) const APPLICATION_EXTENSIONS: &[WebFeaturePackage] = &[
    WebFeaturePackage {
        feature: "clerk",
        section: "dependencies",
        package: "@clerk/nextjs",
    },

    WebFeaturePackage {
        feature: "styled-components",
        section: "dependencies",
        package: "styled-components",
    },
    WebFeaturePackage {
        feature: "styled-components",
        section: "devDependencies",
        package: "@types/styled-components",
    },
    WebFeaturePackage {
        feature: "commitlint",
        section: "devDependencies",
        package: "@commitlint/cli",
    },
    WebFeaturePackage {
        feature: "commitlint",
        section: "devDependencies",
        package: "@commitlint/config-conventional",
    },
    WebFeaturePackage {
        feature: "rest",
        section: "dependencies",
        package: "express",
    },
    WebFeaturePackage {
        feature: "graphql",
        section: "dependencies",
        package: "@apollo/server",
    },
    WebFeaturePackage {
        feature: "graphql",
        section: "dependencies",
        package: "graphql",
    },
    WebFeaturePackage {
        feature: "graphql",
        section: "dependencies",
        package: "@as-integrations/next",
    },
    WebFeaturePackage {
        feature: "trpc",
        section: "dependencies",
        package: "@trpc/server",
    },
    WebFeaturePackage {
        feature: "storybook",
        section: "devDependencies",
        package: "@storybook/react",
    },
    WebFeaturePackage {
        feature: "storybook",
        section: "devDependencies",
        package: "@storybook/addon-essentials",
    },
    WebFeaturePackage {
        feature: "i18n",
        section: "devDependencies",
        package: "i18next",
    },
    WebFeaturePackage {
        feature: "i18n",
        section: "devDependencies",
        package: "react-i18next",
    },
    WebFeaturePackage {
        feature: "i18n",
        section: "devDependencies",
        package: "i18next-browser-languagedetector",
    },
    WebFeaturePackage {
        feature: "i18n",
        section: "devDependencies",
        package: "i18next-http-backend",
    },
];
