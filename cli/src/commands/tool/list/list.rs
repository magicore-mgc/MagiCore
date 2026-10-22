use super::super::managed_roots::managed_roots;
use super::super::record_validation::{read_pending_if_present, read_record_if_present};
use anyhow::{Result, bail};
use std::fs;

pub(in crate::commands::tool) fn list() -> Result<()> {
    let (_data_root, tools_root, _bin_root) = managed_roots(false)?;
    let entries = match fs::read_dir(&tools_root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            println!("No managed CLI tools installed.");
            return Ok(());
        }
        Err(error) => return Err(error.into()),
    };
    let mut records = Vec::new();
    let mut pending_installs = Vec::new();
    for entry in entries {
        let entry = entry?;
        let kind = entry.file_type()?;
        if kind.is_symlink() || !kind.is_dir() {
            bail!(
                "managed tools directory contains an unsafe entry '{}'; refusing to follow it",
                entry.path().display()
            );
        }
        let root = entry.path();
        match read_record_if_present(&root)? {
            Some(record) => records.push(record),
            None => match read_pending_if_present(&root)? {
                Some(pending) => pending_installs.push(pending),
                None => bail!(
                    "unrecognized managed tool data at '{}'; inspect it before removing it",
                    root.display()
                ),
            },
        }
    }
    records.sort_by(|left, right| left.binary.cmp(&right.binary));
    pending_installs.sort_by(|left, right| left.package.cmp(&right.package));
    if records.is_empty() && pending_installs.is_empty() {
        println!("No managed CLI tools installed.");
    } else {
        for record in records {
            println!(
                "{}\t{}:{}@{}",
                record.binary, record.source, record.package, record.installed_version
            );
        }
        for pending in pending_installs {
            println!(
                "INCOMPLETE\t{}:{}@{}\tretry `mgc tool install npm:{}@{}` to recover",
                pending.source,
                pending.package,
                pending.selector,
                pending.package,
                pending.selector
            );
        }
    }
    Ok(())
}
