//! Catalog integrity: every record is complete, unique and self-consistent.
//! Tính toàn vẹn catalog: mọi record đầy đủ, không trùng và tự nhất quán.

use super::super::super::types::{InitializerRecord, Layout};
use super::super::{BACKEND, FRONTEND};

fn all_records() -> Vec<(&'static str, InitializerRecord)> {
    FRONTEND
        .iter()
        .chain(BACKEND.iter())
        .map(|(id, record)| (*id, *record))
        .collect()
}

#[test]
fn every_record_names_a_package_binary_and_identity() {
    for (id, record) in all_records() {
        assert!(!record.package.is_empty(), "{id}: empty package");
        assert!(!record.binary.is_empty(), "{id}: empty binary");
        assert!(!record.identity.is_empty(), "{id}: empty identity");
    }
}

#[test]
fn framework_ids_are_unique_within_a_mode() {
    let mut seen = std::collections::HashSet::new();
    for (id, _) in FRONTEND.iter() {
        assert!(seen.insert(*id), "duplicate frontend id '{id}'");
    }
    let mut seen = std::collections::HashSet::new();
    for (id, _) in BACKEND.iter() {
        assert!(seen.insert(*id), "duplicate backend id '{id}'");
    }
}

#[test]
fn every_record_names_the_project_argument_once() {
    // A record with no `Project` slot cannot receive the user's project name;
    // several would produce an ambiguous invocation.
    // Record không có slot `Project` không nhận được tên project; nhiều slot
    // sẽ tạo lời gọi mơ hồ.
    for (id, record) in all_records() {
        let project_slots = record
            .args
            .iter()
            .filter(|slot| matches!(slot, super::super::super::types::ArgSlot::Project))
            .count();
        assert_eq!(project_slots, 1, "{id}: expected exactly one Project slot");
    }
}

#[test]
fn skip_install_flags_are_present_for_package_managers() {
    // Dependencies must be installed by `mgc install` so that `mgc.lock` is the
    // only dependency truth. A record that lets the initializer install leaves
    // an undescribed graph.
    // Dependency phải do `mgc install` cài để `mgc.lock` là nguồn sự thật duy
    // nhất. Record để initializer tự cài sẽ tạo graph không mô tả được.
    // Initializers that never install on their own (`create-vite`, `sv
    // create`) need no suppression flag; the ones that would must pass one.
    // Initializer vốn không tự install (`create-vite`, `sv create`) không cần
    // cờ; loại có tự install thì bắt buộc phải có.
    const NEVER_INSTALLS: &[&str] = &["create-vite", "sv", "express-generator", "fastify-cli"];
    let skip_flags = ["--skip-install", "--no-install", "--noInstall", "--install"];
    for (id, record) in all_records() {
        if NEVER_INSTALLS.contains(&record.package) {
            continue;
        }
        let literals: Vec<&str> = record
            .args
            .iter()
            .filter_map(|slot| match slot {
                super::super::super::types::ArgSlot::Literal(value) => Some(*value),
                super::super::super::types::ArgSlot::Project => None,
            })
            .collect();
        let declares_install = skip_flags.iter().any(|flag| literals.contains(flag));
        assert!(
            declares_install,
            "{id}: must pass an install-suppression flag so mgc owns the graph"
        );
    }
}

#[test]
fn every_record_generates_a_nested_project() {
    // In-place generation would write into the user's current directory, which
    // conflicts with the existing-directory guard.
    // Sinh tại chỗ sẽ ghi vào thư mục hiện tại của user, xung đột với guard
    // chống ghi đè.
    for (id, record) in all_records() {
        assert_eq!(
            record.layout,
            Layout::NestedProject,
            "{id}: in-place generation is not supported by the create guard"
        );
    }
}
