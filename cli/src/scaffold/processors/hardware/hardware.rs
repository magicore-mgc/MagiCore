//! Stable HardwareProcessor API and output dispatch.
//! API HardwareProcessor ổn định và điều phối output.

use std::path::Path;

use anyhow::Result;

#[path = "bench/mod.rs"]
mod bench;
#[path = "optimizer/mod.rs"]
mod optimizer;

pub struct HardwareProcessor;

impl HardwareProcessor {
    pub fn files(target: &Path, name: &str, framework: &str) -> Result<()> {
        if framework == "optimizer" {
            optimizer::write(target, name)
        } else {
            bench::write(target, name)
        }
    }
}
