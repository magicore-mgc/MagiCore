//! Writes the Pulumi project manifest and package dependencies. — Ghi manifest dự án Pulumi và dependency package.

use std::path::Path;

use mgc_types::MgResult;

pub(in crate::scaffold) async fn scaffold_pulumi(name: &str, dir: &Path) -> MgResult<()> {
    let yaml = format!("name: {}\nruntime: nodejs\n", name);
    std::fs::write(dir.join("Pulumi.yaml"), yaml)?;

    let pkg = format!(
        "{{\"name\":\"{}\",\"dependencies\":{{\"@pulumi/aws\":\"^6.0.0\"}}}}",
        name
    );
    std::fs::write(dir.join("package.json"), pkg)?;
    Ok(())
}
