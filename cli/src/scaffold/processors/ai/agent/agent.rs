//! Writes the agent output for generated AI projects.
//! Ghi output agent cho dự án AI được sinh.

use std::path::Path;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::ai::implementation) fn write(
    target: &Path,
) -> super::super::error::AiScaffoldResult<()> {
    write_file(
        &target.join("src").join("agent.py"),
        r#"'''MagiCore AI agent entry point.'''

from mgc_model_runtime import ModelRuntimeConfig


class AIAgent:
    def __init__(self) -> None:
        self.runtime = ModelRuntimeConfig()

    def run(self, prompt: str) -> str:
        print(f"Executing agent with prompt: {prompt[:50]}...")
        return "Agent processed successfully!"


if __name__ == "__main__":
    agent = AIAgent()
    print(agent.run("Hello MagiCore AI"))
"#,
    )?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
