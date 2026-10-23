//! Repair dangling symlinks inside a materialized node_modules tree.
//! Sửa symlink hỏng trong cây node_modules đã materialize.

use crate::install::link_tree::link_package_tree;
use mgc_types::MgResult;
use std::path::Path;

pub fn repair_dangling_symlinks(node_modules: &Path) -> MgResult<()> {
    let vstore_root = node_modules.join(".magicore");
    let mut fixed = 0usize;
    let mut stack = vec![node_modules.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = match std::fs::read_dir(&dir) {
            Ok(entries) => entries,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let is_symlink = match entry.file_type() {
                Ok(ft) => ft.is_symlink(),
                Err(_) => false,
            };
            if is_symlink {
                if path.exists() {
                    continue;
                }
                let parent = match path.parent() {
                    Some(parent) => parent,
                    None => continue,
                };
                let package_dir = parent.join(path.file_name().unwrap_or_default());
                let name_in_vstore = package_dir
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                let vstore_pkg = vstore_root.join(name_in_vstore);
                if vstore_pkg.exists() {
                    let _ = std::fs::remove_file(&path);
                    link_package_tree(&vstore_pkg, &path)?;
                    fixed += 1;
                }
            } else if path.is_dir() && path != vstore_root {
                stack.push(path);
            }
        }
    }
    if fixed > 0 {
        eprintln!("[magicore] repair: re-linked {} dangling symlink(s)", fixed);
    }
    Ok(())
}
