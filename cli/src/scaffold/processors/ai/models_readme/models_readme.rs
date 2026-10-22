//! Writes the models readme output for generated AI projects.
//! Ghi output models readme cho dự án AI được sinh.

use std::path::Path;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::ai::implementation) fn write(
    target: &Path,
) -> super::super::error::AiScaffoldResult<()> {
    write_file(
        &target.join("models").join("README.md"),
        "# AI Models Directory\n\nManage model artifacts with `mgc model pull hf://org/repo`.\n",
    )?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
