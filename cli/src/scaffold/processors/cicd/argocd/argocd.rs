//! Argo CD output implementation.
//! Triển khai output Argo CD.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::cicd::implementation) fn write(target: &Path) -> Result<()> {
    write_file(
        &target.join("argocd").join("application.yaml"),
        "apiVersion: argoproj.io/v1alpha1\nkind: Application\nmetadata:\n  name: magicore-app\nspec: {}\n",
    )?;
    Ok(())
}
