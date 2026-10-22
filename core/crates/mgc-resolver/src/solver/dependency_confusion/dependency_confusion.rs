use std::collections::HashMap;

use super::types::DepInfo;

/// Check for dependency confusion attacks:
/// - Workspace package vs external dep same name.
/// - Scoped package wrong registry.
/// - Package from untrusted registry.
/// - Typosquatting against known popular packages.
pub fn check_dependency_confusion(
    workspace_packages: &[String],
    dependencies: &[DepInfo],
    scoped_registries: &HashMap<String, String>,
    trusted_registries: &[String],
) -> Vec<String> {
    let mut warnings = Vec::new();

    // Top 50 most depended-upon npm packages (for typosquat detection)
    const TOP_NPM: &[&str] = &[
        "lodash",
        "chalk",
        "react",
        "express",
        "axios",
        "moment",
        "uuid",
        "tslib",
        "commander",
        "prettier",
        "typescript",
        "eslint",
        "webpack",
        "babel",
        "jest",
        "mocha",
        "sinon",
        "async",
        "request",
        "body-parser",
        "cors",
        "debug",
        "dotenv",
        "glob",
        "http-errors",
        "iconv-lite",
        "isarray",
        "js-yaml",
        "json5",
        "jsonwebtoken",
        "ms",
        "node-fetch",
        "once",
        "path-to-regexp",
        "qs",
        "raw-body",
        "readable-stream",
        "safe-buffer",
        "semver",
        "send",
        "serve-static",
        "setprototypeof",
        "statuses",
        "supports-color",
        "through2",
        "underscore",
        "yargs",
        "vue",
        "next",
        "nuxt",
    ];

    for dep in dependencies {
        if workspace_packages.contains(&dep.name) && dep.version.is_some() {
            warnings.push(format!(
                    "Dependency confusion: '{}' is both workspace package and external dep. Use \"workspace:*\".", dep.name
                ));
        }
        // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
        if dep.name.starts_with('@')
            && let Some(scope) = dep.name.split('/').next()
            && let Some(expected) = scoped_registries.get(scope)
            && dep.registry.as_deref() != Some(expected.as_str())
        {
            warnings.push(format!(
                "Dependency confusion: '{}' should resolve from '{}' but resolves from '{}'",
                dep.name,
                expected,
                dep.registry.as_deref().unwrap_or("public npm")
            ));
        }
        // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
        if !trusted_registries.is_empty()
            && let Some(ref reg) = dep.registry
            && !trusted_registries.contains(reg)
        {
            warnings.push(format!(
                "Dependency confusion: '{}' from '{}' not in trusted registries",
                dep.name, reg
            ));
        }
        // Typosquat check: Levenshtein distance against top npm packages
        if !dep.name.starts_with('@') && !dep.name.contains('/') {
            for &popular in TOP_NPM {
                let dist = levenshtein_distance(&dep.name, popular);
                if dist == 1 && dep.name.len() >= 4 {
                    warnings.push(format!(
                        "Typosquat warning: '{}' (distance 1 from '{}')",
                        dep.name, popular
                    ));
                } else if dist == 2 && dep.name.len() >= 6 {
                    warnings.push(format!(
                        "Typosquat warning: '{}' (distance 2 from '{}')",
                        dep.name, popular
                    ));
                }
            }
        }
    }
    warnings
}

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_len = a.len();
    let b_len = b.len();
    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }
    let mut prev: Vec<usize> = (0..=b_len).collect();
    let mut curr = vec![0; b_len + 1];
    for (i, ca) in a.chars().enumerate() {
        curr[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            curr[j + 1] =
                std::cmp::min(std::cmp::min(curr[j] + 1, prev[j + 1] + 1), prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[b_len]
}
