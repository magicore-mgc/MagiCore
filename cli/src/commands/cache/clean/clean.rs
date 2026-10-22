//! Clean explicitly selected cache paths.
//! Xóa cache được chọn khi có xác nhận `--yes`.

use crate::commands::cache::{CacheEntry, filesystem::remove_path, status::print_status};
use anyhow::Result;

pub(in crate::commands::cache) fn clean(entries: &[CacheEntry], yes: bool) -> Result<()> {
    if !yes {
        println!("Refusing to clean cache without --yes.");
        print_status(entries)?;
        return Ok(());
    }

    for entry in entries {
        if !entry.removable || !entry.path.exists() {
            println!("skip\t{}\t{}", entry.label, entry.path.display());
            continue;
        }
        remove_path(&entry.path)?;
        println!("removed\t{}\t{}", entry.label, entry.path.display());
    }
    Ok(())
}
