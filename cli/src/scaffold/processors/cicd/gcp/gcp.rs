//! Google Cloud Build output implementation.
//! Triển khai output Google Cloud Build.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::cicd::implementation) fn write(target: &Path) -> Result<()> {
    write_file(
        &target.join("cloudbuild.yaml"),
        "steps:\n  - name: gcr.io/cloud-builders/gcloud\n    entrypoint: bash\n    args:\n      - -ceu\n      - echo 'MagiCore Google Cloud Build scaffold'\noptions:\n  logging: CLOUD_LOGGING_ONLY\n",
    )?;
    Ok(())
}
