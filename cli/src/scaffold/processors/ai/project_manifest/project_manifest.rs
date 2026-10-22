//! Writes the project manifest output for generated AI projects.
//! Ghi output project manifest cho dự án AI được sinh.

use std::path::Path;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::ai::implementation) fn write(
    target: &Path,
    slug: &str,
    framework: &str,
) -> super::super::error::AiScaffoldResult<()> {
    write_file(
        &target.join("pyproject.toml"),
        &format!(
            "[build-system]\n\
                 requires = [\"setuptools>=80\"]\n\
                 build-backend = \"setuptools.build_meta\"\n\n\
                 [project]\n\
                 name = \"{slug}\"\n\
                 version = \"0.1.0\"\n\
                 description = \"MagiCore AI Project\"\n\
                 requires-python = \">=3.11\"\n\
                 dependencies = []\n\n\
                 [dependency-groups]\n\
                 dev = [\n\
                     \"pytest==9.1.1\",\n\
                     \"build==1.6.1\",\n\
                     \"setuptools==84.0.0\",\n\
                 ]\n\n\
                 [project.optional-dependencies]\n\
                 model = [\n\
                     \"torch>=2.4.0\",\n\
                     \"transformers>=4.44.0\",\n\
                     \"accelerate>=0.33.0\",\n\
                     \"safetensors>=0.4.4\",\n\
                 ]\n\n\
                 [tool.magicore]\n\
                 framework = \"{framework}\"\n\
                 core = \"ai\"\n\n\
                 [tool.setuptools]\n\
                 py-modules = [\"agent\", \"mgc_model_runtime\"]\n\
                 package-dir = {{\"\" = \"src\"}}\n"
        ),
    )?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
