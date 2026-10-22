//! Hardware optimizer marker implementation.
//! Triển khai marker hardware optimizer.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::{slugify, write_file};

pub(in crate::scaffold::processors::hardware::implementation) fn write(
    target: &Path,
    name: &str,
) -> Result<()> {
    write_file(
        &target.join("optimizer.json"),
        &format!(
            "{{\n  \"name\": \"{}\",\n  \"kind\": \"optimizer\",\n  \"description\": \"MagiCore hardware optimizer package — run `mgc optimizer` here to generate hardware-profiled configs into .mgc-optimizer/\"\n}}\n",
            slugify(name)
        ),
    )?;
    Ok(())
}
