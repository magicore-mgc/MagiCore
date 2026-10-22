//! Ordered application foundation package records.
//! Bản ghi package application foundation được giữ đúng thứ tự.

use super::super::WebFeaturePackage;

pub(in crate::commands::core::web::implementation::feature_catalog::implementation) const APPLICATION_FOUNDATION: &[WebFeaturePackage] = &[
    WebFeaturePackage {
        feature: "prisma",
        section: "dependencies",
        package: "@prisma/client",
    },
    WebFeaturePackage {
        feature: "prisma",
        section: "devDependencies",
        package: "prisma",
    },
    WebFeaturePackage {
        feature: "vitest",
        section: "devDependencies",
        package: "vitest",
    },
    WebFeaturePackage {
        feature: "eslint",
        section: "devDependencies",
        package: "eslint",
    },
    WebFeaturePackage {
        feature: "eslint",
        section: "devDependencies",
        package: "eslint-config-next",
    },
    WebFeaturePackage {
        feature: "prettier",
        section: "devDependencies",
        package: "prettier",
    },
    WebFeaturePackage {
        feature: "tailwindcss",
        section: "devDependencies",
        package: "@tailwindcss/postcss",
    },
    WebFeaturePackage {
        feature: "tailwindcss",
        section: "devDependencies",
        package: "@tailwindcss/vite",
    },
    WebFeaturePackage {
        feature: "postgres",
        section: "dependencies",
        package: "pg",
    },
    WebFeaturePackage {
        feature: "zustand",
        section: "dependencies",
        package: "zustand",
    },
    WebFeaturePackage {
        feature: "tanstack-query",
        section: "dependencies",
        package: "@tanstack/react-query",
    },
    WebFeaturePackage {
        feature: "zod",
        section: "dependencies",
        package: "zod",
    },
    WebFeaturePackage {
        feature: "shadcn",
        section: "dependencies",
        package: "clsx",
    },
    WebFeaturePackage {
        feature: "shadcn",
        section: "dependencies",
        package: "tailwind-merge",
    },
    WebFeaturePackage {
        feature: "shadcn",
        section: "dependencies",
        package: "class-variance-authority",
    },
    WebFeaturePackage {
        feature: "nextauth",
        section: "dependencies",
        package: "next-auth",
    },
    WebFeaturePackage {
        feature: "playwright",
        section: "devDependencies",
        package: "@playwright/test",
    },
    WebFeaturePackage {
        feature: "husky",
        section: "devDependencies",
        package: "husky",
    },
    WebFeaturePackage {
        feature: "lint-staged",
        section: "devDependencies",
        package: "lint-staged",
    },
    WebFeaturePackage {
        feature: "biome",
        section: "devDependencies",
        package: "@biomejs/biome",
    },
    WebFeaturePackage {
        feature: "sass",
        section: "devDependencies",
        package: "sass",
    },
    WebFeaturePackage {
        feature: "unocss",
        section: "devDependencies",
        package: "unocss",
    },
    WebFeaturePackage {
        feature: "daisyui",
        section: "devDependencies",
        package: "daisyui",
    },
    WebFeaturePackage {
        feature: "redux",
        section: "dependencies",
        package: "@reduxjs/toolkit",
    },
    WebFeaturePackage {
        feature: "redux",
        section: "dependencies",
        package: "react-redux",
    },
    WebFeaturePackage {
        feature: "jest",
        section: "devDependencies",
        package: "jest",
    },
    WebFeaturePackage {
        feature: "jest",
        section: "devDependencies",
        package: "@testing-library/react",
    },
    WebFeaturePackage {
        feature: "jest",
        section: "devDependencies",
        package: "@testing-library/jest-dom",
    },
    WebFeaturePackage {
        feature: "jest",
        section: "devDependencies",
        package: "jest-environment-jsdom",
    },
    WebFeaturePackage {
        feature: "cypress",
        section: "devDependencies",
        package: "cypress",
    },
    WebFeaturePackage {
        feature: "drizzle",
        section: "dependencies",
        package: "drizzle-orm",
    },
    WebFeaturePackage {
        feature: "drizzle",
        section: "devDependencies",
        package: "drizzle-kit",
    },













];
