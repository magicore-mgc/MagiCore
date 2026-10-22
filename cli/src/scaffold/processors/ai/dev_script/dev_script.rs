//! Writes the dev script output for generated AI projects.
//! Ghi output dev script cho dự án AI được sinh.

use std::path::Path;

use crate::scaffold::processors::write_file;

pub(in crate::scaffold::processors::ai::implementation) fn write(
    target: &Path,
    entrypoint: &str,
) -> super::super::error::AiScaffoldResult<()> {
    write_file(
        &target.join("scripts").join("run_dev.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nif [[ -f .mgc-optimizer/ai_runtime.env ]]; then\n  source .mgc-optimizer/ai_runtime.env\nfi\npython3 {entrypoint}\n"
        ),
    )?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
