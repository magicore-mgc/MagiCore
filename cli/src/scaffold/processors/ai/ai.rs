//! AI scaffold coordinator preserves the stable API and output order.
//! Coordinator AI scaffold giữ API ổn định và thứ tự ghi output.

use anyhow::Result;
use std::path::Path;

#[path = "agent/mod.rs"]
mod agent;
#[path = "agent_smoke_test/mod.rs"]
mod agent_smoke_test;
#[path = "dev_script/mod.rs"]
mod dev_script;
#[path = "error/mod.rs"]
mod error;
#[path = "mcp_server/mod.rs"]
mod mcp_server;
#[path = "mcp_smoke_test/mod.rs"]
mod mcp_smoke_test;
#[path = "model_configuration/mod.rs"]
mod model_configuration;
#[path = "model_runtime/mod.rs"]
mod model_runtime;
#[path = "models_readme/mod.rs"]
mod models_readme;
#[path = "project_manifest/mod.rs"]
mod project_manifest;

pub struct AiProcessor;

impl AiProcessor {
    pub fn files(target: &Path, name: &str, framework: &str) -> Result<()> {
        let package = super::super::slugify(name).replace('-', "_");
        let slug = super::super::slugify(name);

        project_manifest::write(target, &slug, framework)?;

        // Defaults stay inert until a runtime consumes them.
        // Giá trị mặc định không tuyên bố tối ưu khi chưa có runtime áp dụng.
        model_configuration::write(target, &slug)?;
        model_runtime::write(target)?;

        if framework == "mcp-server" {
            mcp_server::write(target, &package)?;
        } else {
            agent::write(target)?;
        }

        let entrypoint = if framework == "mcp-server" {
            "src/server.py"
        } else {
            "src/agent.py"
        };
        dev_script::write(target, entrypoint)?;

        // Keep generated smoke tests in the scaffold; never fabricate them at test time.
        // Giữ smoke test trong scaffold; không tạo giả test lúc chạy kiểm thử.
        if framework == "mcp-server" {
            mcp_smoke_test::write(target)?;
        } else {
            agent_smoke_test::write(target)?;
        }

        models_readme::write(target)?;
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
