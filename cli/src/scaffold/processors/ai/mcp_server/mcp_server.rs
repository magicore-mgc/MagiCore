//! Writes the mcp server output for generated AI projects.
//! Ghi output mcp server cho dự án AI được sinh.

use std::path::Path;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::ai::implementation) fn write(
    target: &Path,
    package: &str,
) -> super::super::error::AiScaffoldResult<()> {
    write_file(
        &target.join("src").join("server.py"),
        &format!(
            r#"'''MagiCore MCP server entry point.'''

from mgc_model_runtime import ModelRuntimeConfig


def main() -> None:
    config = ModelRuntimeConfig()
    print("MagiCore MCP Server ({package}) initialized!", config.target_bits)


if __name__ == "__main__":
    main()
"#
        ),
    )?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
