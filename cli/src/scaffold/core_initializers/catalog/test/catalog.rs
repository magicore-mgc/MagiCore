//! Catalog integrity for the non-Web initializers.
//! Tính toàn vẹn catalog initializer không phải Web.

use super::super::RECORDS;

#[test]
fn every_record_names_a_program_probe_and_marker() {
    for record in RECORDS {
        assert!(!record.program.is_empty(), "{}: program", record.framework);
        assert!(!record.probe.is_empty(), "{}: probe", record.framework);
        assert!(!record.marker.is_empty(), "{}: marker", record.framework);
        assert!(
            !record.install_hint.is_empty(),
            "{}: install hint",
            record.framework
        );
    }
}

#[test]
fn ecosystem_and_framework_pairs_are_unique() {
    let mut seen = std::collections::HashSet::new();
    for record in RECORDS {
        assert!(
            seen.insert((record.ecosystem, record.framework)),
            "duplicate {}/{}",
            record.ecosystem,
            record.framework
        );
    }
}

#[test]
fn every_ecosystem_is_one_magicore_core() {
    // A record may only claim an ecosystem MagiCore actually has a core for.
    // Record chỉ được nhận ecosystem mà MagiCore thật sự có core.
    const ECOSYSTEMS: &[&str] = &["web", "game", "iot", "clo", "cicd", "lib", "ai", "app"];
    for record in RECORDS {
        assert!(
            ECOSYSTEMS.contains(&record.ecosystem),
            "unknown ecosystem '{}'",
            record.ecosystem
        );
    }
}

#[test]
fn no_record_delegates_to_a_package_manager() {
    // The program must be the framework's own tool, not another package
    // manager standing in for one.
    // Chương trình phải là tool của chính framework, không phải package manager
    // khác đứng thay.
    const FORBIDDEN: &[&str] = &["npm", "npx", "pnpm", "yarn", "bun", "bunx"];
    for record in RECORDS {
        assert!(
            !FORBIDDEN.contains(&record.program),
            "{} delegates to a package manager",
            record.framework
        );
    }
}
