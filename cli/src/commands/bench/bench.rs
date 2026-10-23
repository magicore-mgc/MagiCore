//! mgc bench — benchmark install layers (C3)
//! Bật profile có sẵn (MAGICORE_WEB_PROFILE_INSTALL) + đo wall-time mỗi layer.
//! Không thêm crate profiling — tái dùng profile nhúng của adapter.

use anyhow::Result;
use clap::Args;

#[derive(Args, Debug, Clone)]
pub struct BenchArgs {
    /// Packages để add + install (như mgc install)
    #[arg(help = "package@version (multiple packages)")]
    packages: Vec<String>,

    #[arg(long, help = "force fresh resolve (skip lockfile)")]
    no_lock: bool,

    #[arg(
        long,
        help = "Allow dependency lifecycle scripts during benchmark install"
    )]
    allow_scripts: bool,
}

pub async fn handle(args: BenchArgs) -> Result<()> {
    if args.no_lock {
        return Err(crate::error::bench_no_lock_unsafe());
    }

    // SAFETY: set once at command entry, before bench spawns any thread that
    // might read the flag — single writer, no concurrent mutation.
    // AN TOÀN: ghi 1 lần ở entry, trước khi bench spawn thread đọc flag.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var("MAGICORE_WEB_PROFILE_INSTALL", "1");
    }

    let started = std::time::Instant::now();

    crate::commands::install::run(
        args.packages,
        None,
        !args.allow_scripts,
        args.allow_scripts,
        false,
        false,
    )
    .await?;

    let total_ms = started.elapsed().as_millis() as u64;
    eprintln!("[magicore:bench] total={}ms", total_ms);
    Ok(())
}
