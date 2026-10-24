//! Policy tests: no package manager may stand in for an initializer.
//! Test policy: không package manager nào được thay initializer.

use super::super::super::super::types::{ArgSlot, InitializerRecord, Layout, VersionPolicy};
use super::super::{Refusal, check};

fn record(package: &'static str, binary: &'static str) -> InitializerRecord {
    InitializerRecord {
        package,
        binary,
        identity: "test:default",
        args: &[ArgSlot::Project],
        layout: Layout::NestedProject,
        version: VersionPolicy::Latest,
    }
}

#[test]
fn package_managers_are_refused() {
    for pm in ["npm", "npx", "pnpm", "yarn", "bun", "bunx", "deno"] {
        let refusal = check(&record(pm, "run")).expect_err("a PM cannot be an initializer");
        assert_eq!(refusal, Refusal::PackageManager { package: pm });
    }
}

#[test]
fn a_scoped_framework_package_is_accepted() {
    check(&record("@angular/cli", "ng")).expect("@angular/cli is a legitimate initializer");
    check(&record("@nestjs/cli", "nest")).expect("@nestjs/cli is a legitimate initializer");
    check(&record("create-vite", "create-vite")).expect("create-vite is legitimate");
}

#[test]
fn a_package_name_that_could_escape_the_scope_is_refused() {
    for name in ["@scope/../../evil", "pkg/../evil", "/absolute", "pkg/"] {
        let refusal = check(&record(name, "run")).expect_err("malformed package id");
        assert_eq!(refusal, Refusal::InvalidPackage { package: name });
    }
}

#[test]
fn a_binary_name_that_could_escape_the_package_is_refused() {
    for binary in ["../bin/evil", "bin/evil", ".", ""] {
        let refusal = check(&record("create-vite", binary)).expect_err("malformed binary");
        assert_eq!(refusal, Refusal::InvalidBinary { binary });
    }
}
