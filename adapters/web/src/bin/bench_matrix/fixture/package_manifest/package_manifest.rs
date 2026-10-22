// Benchmark project manifest writer
// Ghi package.json tạm theo direct dependencies của fixture.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn write_package_json(
    root: &Path,
    packages: &[PackageId],
) -> anyhow::Result<()> {
    let dependencies = packages
        .iter()
        .map(|pkg| {
            (
                pkg.name_str().to_string(),
                serde_json::Value::String(format!("^{}", pkg.version())),
            )
        })
        .collect::<serde_json::Map<String, serde_json::Value>>();
    std::fs::write(
        root.join("package.json"),
        serde_json::json!({
            "name": "mgc-bench",
            "version": "0.1.0",
            "private": true,
            "dependencies": dependencies
        })
        .to_string(),
    )?;
    Ok(())
}
