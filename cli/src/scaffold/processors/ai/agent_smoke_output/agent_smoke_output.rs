//! Writes the agent smoke test output for generated AI projects.
//! Ghi output agent smoke test cho dự án AI được sinh.

use std::path::Path;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::ai::implementation) fn write(
    target: &Path,
) -> super::super::error::AiScaffoldResult<()> {
    write_file(
        &target.join("tests").join("test_agent.py"),
        r#"'''Scaffold smoke test: the agent runs end-to-end.'''

import importlib.util
import pathlib
import sys

SRC = pathlib.Path(__file__).resolve().parents[1] / "src"
if str(SRC) not in sys.path:
    sys.path.insert(0, str(SRC))


def _load_agent():
    path = SRC / "agent.py"
    spec = importlib.util.spec_from_file_location("agent", path)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def test_agent_runs() -> None:
    agent = _load_agent().AIAgent()
    assert agent.run("CI") == "Agent processed successfully!"


def test_runtime_config_inert() -> None:
    agent = _load_agent().AIAgent()
    assert agent.runtime.target_bits is None
"#,
    )?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
