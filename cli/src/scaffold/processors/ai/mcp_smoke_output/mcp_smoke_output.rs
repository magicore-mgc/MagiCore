//! Writes the mcp smoke test output for generated AI projects.
//! Ghi output mcp smoke test cho dự án AI được sinh.

use std::path::Path;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::ai::implementation) fn write(
    target: &Path,
) -> super::super::error::AiScaffoldResult<()> {
    write_file(
        &target.join("tests").join("test_server.py"),
        r#"'''Scaffold smoke test: the MCP server entry point imports.'''

import importlib.util
import pathlib
import sys

SRC = pathlib.Path(__file__).resolve().parents[1] / "src"
if str(SRC) not in sys.path:
    sys.path.insert(0, str(SRC))


def test_server_module_imports() -> None:
    path = SRC / "server.py"
    spec = importlib.util.spec_from_file_location("server", path)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    assert hasattr(module, "main")
"#,
    )?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
