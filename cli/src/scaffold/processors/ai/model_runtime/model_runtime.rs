//! Writes the model runtime output for generated AI projects.
//! Ghi output model runtime cho dự án AI được sinh.

use std::path::Path;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::ai::implementation) fn write(
    target: &Path,
) -> super::super::error::AiScaffoldResult<()> {
    write_file(
        &target.join("src").join("mgc_model_runtime.py"),
        "\"\"\"Runtime configuration for optional model optimization.\"\"\"\n\nfrom typing import Optional\n\n\nclass ModelRuntimeConfig:\n    \"\"\"Describe requested optimization without mutating a model.\"\"\"\n\n    def __init__(self, target_bits: Optional[int] = None):\n        self.target_bits = target_bits\n",
    )?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
