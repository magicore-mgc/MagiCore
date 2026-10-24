//! Hardware package markers — MagiCore-owned scaffold output.
//! Marker package Hardware — output scaffold do MagiCore sở hữu.
//!
//! `optimizer` and `bench` are packages MagiCore publishes itself. Unlike
//! framework source, their marker files belong to this repository, so writing
//! them here does not embed any third-party template.
//! `optimizer` và `bench` là package MagiCore tự phát hành. Khác source
//! framework, marker của chúng thuộc repo này nên ghi ở đây không nhúng
//! template bên thứ ba.

use std::path::Path;

use anyhow::Result;

/// Write the marker for one hardware package, refusing unknown ids.
/// Ghi marker cho một package Hardware, từ chối id lạ.
pub(crate) fn write(target: &Path, name: &str, framework: &str) -> Result<()> {
    let slug = crate::scaffold::processor::slugify(name);
    let (file, kind, description) = match framework {
        "optimizer" => (
            "optimizer.json",
            "optimizer",
            "MagiCore hardware optimizer package — run `mgc optimizer` here to generate hardware-profiled configs into .mgc-optimizer/",
        ),
        "bench" => (
            "bench.json",
            "bench",
            "MagiCore hardware benchmark package — run `mgc bench` here to time installs",
        ),
        other => {
            return Err(anyhow::anyhow!(
                "Unsupported hardware framework '{other}': expected 'optimizer' or 'bench'"
            ));
        }
    };
    let body = format!(
        "{{\n  \"name\": \"{slug}\",\n  \"kind\": \"{kind}\",\n  \"description\": \"{description}\"\n}}\n"
    );
    std::fs::create_dir_all(target)?;
    std::fs::write(target.join(file), body)?;
    Ok(())
}
