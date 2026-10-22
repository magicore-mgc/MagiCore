//! AWS CodeBuild output implementation.
//! Triển khai output AWS CodeBuild.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::cicd::implementation) fn write(target: &Path) -> Result<()> {
    write_file(
        &target.join("buildspec.yml"),
        "version: 0.2\nphases:\n  build:\n    commands:\n      - echo 'MagiCore AWS CodeBuild scaffold'\nartifacts:\n  files:\n    - '**/*'\n",
    )?;
    Ok(())
}
