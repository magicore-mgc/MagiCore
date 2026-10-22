//! Writes the model configuration output for generated AI projects.
//! Ghi output model configuration cho dự án AI được sinh.

use std::path::Path;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::ai::implementation) fn write(
    target: &Path,
    slug: &str,
) -> super::super::error::AiScaffoldResult<()> {
    write_file(
        &target.join("configs").join("model_config.json"),
        &format!(
            "{{\n\
                  \"model_name\": \"{slug}\",\n\
                  \"quantization\": null,\n\
                  \"sharding\": {{\n\
                    \"enabled\": false,\n\
                    \"max_memory_per_gpu\": null,\n\
                    \"offload_to_cpu\": false\n\
                  }}\n\
                }}\n"
        ),
    )?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
