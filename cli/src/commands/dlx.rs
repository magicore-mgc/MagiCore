//! `mgc dlx` user-facing command wrapper.
//! Lớp lệnh `mgc dlx`; cài đặt và chạy package do package runtime dùng chung xử lý.

use anyhow::Result;

use crate::commands::package_runtime::{PackageRunPurpose, run_registry_package_in};

pub async fn run(
    package: String,
    selected_bin: Option<String>,
    allow_scripts: bool,
    args: Vec<String>,
) -> Result<()> {
    let execution_cwd = std::env::current_dir()?;
    run_registry_package_in(
        package,
        selected_bin,
        allow_scripts,
        args,
        &execution_cwd,
        PackageRunPurpose::Dlx,
    )
    .await
    .map(|_| ())
}
