//! Hardware benchmark marker implementation.
//! Triển khai marker benchmark hardware.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::{slugify, write_file};

pub(in crate::scaffold::processors::hardware::implementation) fn write(
    target: &Path,
    name: &str,
) -> Result<()> {
    // "bench" and any future hardware package share the
    // benchmark-harness marker (unknown names are rejected
    // earlier by create-hardware/add-hardware validation).
    // (bench và package phần cứng khác dùng chung marker.)
    write_file(
        &target.join("bench.json"),
        &format!(
            "{{\n  \"name\": \"{}\",\n  \"kind\": \"bench\",\n  \"description\": \"MagiCore hardware benchmark package — run `mgc bench` here to time installs\"\n}}\n",
            slugify(name)
        ),
    )?;
    Ok(())
}
