//! AI framework variants and stable presentation mappings.
//! Các variant framework AI và ánh xạ trình bày ổn định.

/// Frameworks currently recognized by the AI adapter.
/// Framework hiện được adapter AI nhận diện.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiFramework {
    PythonAgent,
    McpServer,
}

impl AiFramework {
    /// Return the canonical config string for this framework.
    /// Trả chuỗi config chuẩn của framework này.
    pub fn as_str(&self) -> &'static str {
        match self {
            AiFramework::PythonAgent => "python-agent",
            AiFramework::McpServer => "mcp-server",
        }
    }

    /// Return the existing default entry-script path for this framework.
    /// Trả đường dẫn entry script mặc định hiện tại của framework này.
    pub fn entry_script(&self) -> &'static str {
        match self {
            AiFramework::PythonAgent => "src/agent.py",
            AiFramework::McpServer => "server.py",
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
