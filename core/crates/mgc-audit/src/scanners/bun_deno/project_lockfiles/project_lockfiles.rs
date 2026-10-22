//! Read all present Bun and Deno lockfiles — đọc mọi lock Bun và Deno đang có.
//! This helper handles runtime lockfiles only; it does not parse `mgc.lock`.
//! Helper này chỉ xử lý lockfile runtime; không parse `mgc.lock`.

use crate::scanners::bun_deno::{BunDenoRead, read_bun_lock, read_deno_lock};
use crate::scanners::read_bounded_project_input;
use mgc_types::MgResult;
use std::path::Path;

/// Read and merge each present `bun.lock` and `deno.lock`, in that order.
/// This helper does not read `mgc.lock`.
/// Đọc và gộp từng file `bun.lock` và `deno.lock` hiện có, theo thứ tự đó.
/// Helper này không đọc `mgc.lock`.
pub fn read_js_lockfiles(project_root: &Path) -> MgResult<BunDenoRead> {
    let mut combined = BunDenoRead::default();
    let bun_path = project_root.join("bun.lock");
    if bun_path.is_file() {
        let raw = read_bounded_project_input(&bun_path, "bun.lock")?;
        let read = read_bun_lock(&raw)?;
        combined.npm_pins.extend(read.npm_pins);
        combined.jsr_skipped.extend(read.jsr_skipped);
    }
    let deno_path = project_root.join("deno.lock");
    if deno_path.is_file() {
        let raw = read_bounded_project_input(&deno_path, "deno.lock")?;
        let read = read_deno_lock(&raw)?;
        combined.npm_pins.extend(read.npm_pins);
        combined.jsr_skipped.extend(read.jsr_skipped);
    }
    Ok(combined)
}
