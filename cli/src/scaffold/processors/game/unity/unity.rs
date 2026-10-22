//! Unity fallback project files — file project fallback Unity.

use std::path::Path;

use anyhow::Result;

use super::super::super::write_file;

pub(in crate::scaffold::processors::game) fn files(target: &Path) -> Result<()> {
    write_file(
        &target.join("Packages").join("manifest.json"),
        PACKAGE_MANIFEST,
    )?;
    write_file(
        &target.join("Assets").join("Scripts").join("Bootstrap.cs"),
        BOOTSTRAP_SOURCE,
    )?;
    Ok(())
}

const PACKAGE_MANIFEST: &str = r#"{
  "dependencies": {}
}
"#;

const BOOTSTRAP_SOURCE: &str = r#"using UnityEngine;

public class Bootstrap : MonoBehaviour
{
    void Start()
    {
        Debug.Log("MagiCore Unity project ready");
    }
}
"#;
