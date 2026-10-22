//! Collect package pins across solution projects and the root lockfile.
//! Gom pin package từ project trong solution và lockfile root.

use crate::scanners::osv::OsvPin;
use crate::scanners::read_bounded_project_input;
use mgc_types::MgResult;
use std::collections::BTreeSet;
use std::path::Path;

use super::super::lockfile::read_packages_lock;
use super::super::project_path::{canonical_root, resolve_project_path};
use super::super::solution::find_solution_files;

/// Gather deduplicated pins from all sorted solution projects and root lock.
/// Gom pin đã khử trùng từ mọi project solution đã sắp xếp và lock root.
pub fn collect_dotnet_pins(project_root: &Path) -> MgResult<(Vec<OsvPin>, Vec<String>, bool)> {
    let mut pins = Vec::new();
    let mut skipped = Vec::new();
    let mut seen = BTreeSet::new();
    let mut recognized = false;

    let mut solution_paths = find_solution_files(project_root);
    solution_paths.sort();
    for solution_path in &solution_paths {
        recognized = true;
        let solution_text = read_bounded_project_input(solution_path, "solution file")?;
        let projects = super::super::solution::read_solution_projects(&solution_text);
        if projects.is_empty() {
            skipped.push(format!(
                "{} lists no .csproj projects — nothing auditable in this solution",
                solution_path.display()
            ));
            continue;
        }
        for relative_path in &projects {
            let project_path = resolve_project_path(project_root, relative_path);
            let inside_root = project_path
                .as_deref()
                .and_then(|path| path.canonicalize().ok())
                .filter(|absolute| absolute.starts_with(canonical_root(project_root)));
            let Some(project_file) = inside_root
                .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("csproj"))
            else {
                skipped.push(format!(
                    "{relative_path}: cannot resolve a unique .csproj inside the project root — refused"
                ));
                continue;
            };
            let lockfile = project_file
                .parent()
                .map(|directory| directory.join("packages.lock.json"))
                .filter(|path| path.is_file());
            match lockfile {
                Some(path) => {
                    let raw = read_bounded_project_input(&path, "packages.lock.json")?;
                    let (mut new_pins, mut new_skips) = read_packages_lock(&raw)?;
                    drain_new_pins(&mut new_pins, &mut seen, &mut pins);
                    skipped.append(&mut new_skips);
                }
                None => skipped.push(format!(
                    "{relative_path}: no packages.lock.json — run `dotnet restore --locked-mode` in {}",
                    project_file
                        .parent()
                        .map(|directory| directory.display().to_string())
                        .unwrap_or_default()
                )),
            }
        }
    }

    let root_lockfile = project_root.join("packages.lock.json");
    if root_lockfile.is_file() {
        recognized = true;
        let raw = read_bounded_project_input(&root_lockfile, "packages.lock.json")?;
        let (mut new_pins, mut new_skips) = read_packages_lock(&raw)?;
        drain_new_pins(&mut new_pins, &mut seen, &mut pins);
        skipped.append(&mut new_skips);
    }
    Ok((pins, skipped, recognized))
}

fn drain_new_pins(source: &mut Vec<OsvPin>, seen: &mut BTreeSet<String>, output: &mut Vec<OsvPin>) {
    for pin in source.drain(..) {
        if seen.insert(format!("{}@{}", pin.name, pin.version)) {
            output.push(pin);
        }
    }
}
