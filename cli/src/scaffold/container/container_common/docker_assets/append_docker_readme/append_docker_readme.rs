//! Appends the shared Docker run contract to a generated project README.
//! Bổ sung hợp đồng chạy Docker dùng chung vào README của project sinh ra.

use anyhow::{Context, Result};
use std::path::Path;

use crate::scaffold::processor::Scaffolder;

pub(in crate::scaffold::container) fn append_docker_readme(target: &Path) -> Result<()> {
    let path = target.join("README.md");
    let mut readme = match std::fs::read_to_string(&path) {
        Ok(readme) => readme,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::from("# Project\n"),
        Err(error) => return Err(error).with_context(|| format!("read {}", path.display())),
    };
    if readme.contains("docker compose up --build") && readme.contains("copies that same lock") {
        return Ok(());
    }
    if !readme.ends_with('\n') {
        readme.push('\n');
    }
    readme.push_str(concat!(
        "\n## Docker\n\nRun `mgc install` in this directory to create the project lockfile, ",
        "then run `docker compose up --build`. The image copies that same lock ",
        "and uses `mgc install --frozen`; dependency resolution and lifecycle ",
        "remain identical between the host project and container.\n",
    ));
    Scaffolder::write_file(&path, &readme)
}
