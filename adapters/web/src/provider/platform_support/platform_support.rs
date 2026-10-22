//! `platform_support.rs` — platform support.
//! Tương thích nền tảng host.

use crate::native;
use crate::provider::NpmDependencyProvider;
use mgc_types::PackageName;

impl NpmDependencyProvider {
    pub fn current_npm_os() -> &'static str {
        match std::env::consts::OS {
            "macos" => "darwin",
            "windows" => "win32",
            other => other,
        }
    }

    pub fn current_npm_cpu() -> &'static str {
        match std::env::consts::ARCH {
            "x86_64" => "x64",
            "aarch64" => "arm64",
            "x86" => "ia32",
            "powerpc64" => "ppc64",
            "loongarch64" => "loong64",
            other => other,
        }
    }

    pub fn platform_matches(rules: Option<&[String]>, current: &str) -> bool {
        let Some(rules) = rules else {
            return true;
        };
        if rules.is_empty() {
            return true;
        }

        let mut positive = Vec::new();
        let mut negative = Vec::new();
        for rule in rules {
            if let Some(stripped) = rule.strip_prefix('!') {
                negative.push(stripped);
            } else {
                positive.push(rule.as_str());
            }
        }

        if negative.contains(&current) {
            return false;
        }
        if positive.is_empty() {
            true
        } else {
            positive.contains(&current)
        }
    }

    pub fn version_supported(info: &native::npm_registry::VersionInfo) -> bool {
        Self::platform_matches(info.os.as_deref(), Self::current_npm_os())
            && Self::platform_matches(info.cpu.as_deref(), Self::current_npm_cpu())
    }

    pub fn known_optional_native_binary_supported(package: &PackageName) -> Option<bool> {
        let name = package.as_str();
        let os = Self::current_npm_os();
        let cpu = Self::current_npm_cpu();
        let expected = format!("{os}-{cpu}");

        let target = name
            .strip_prefix("@esbuild/")
            .or_else(|| name.strip_prefix("@next/swc-"))
            .or_else(|| name.strip_prefix("@swc/core-"))
            .or_else(|| name.strip_prefix("@typescript/typescript-"))
            .or_else(|| name.strip_prefix("@rollup/rollup-"))
            .or_else(|| name.strip_prefix("@tailwindcss/oxide-"))
            .or_else(|| name.strip_prefix("@openai/codex-"))
            .or_else(|| name.strip_prefix("lightningcss-"))
            .or_else(|| name.strip_prefix("@parcel/watcher-"))?;

        Some(target.starts_with(&expected))
    }
}
