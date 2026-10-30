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

    /// One-line description written into the generated project metadata.
    /// Mô tả một dòng ghi vào metadata project sinh ra.
    pub fn description(&self) -> &'static str {
        match self {
            AiFramework::PythonAgent => "Python agent loop backed by a chat-completions endpoint",
            AiFramework::McpServer => "Model Context Protocol server over stdio JSON-RPC",
        }
    }

    /// Parse a canonical framework id without normalizing unknown strings.
    /// Parse id framework chuẩn, không chuẩn hóa chuỗi lạ.
    pub fn parse(name: &str) -> Option<Self> {
        match name {
            "python-agent" => Some(AiFramework::PythonAgent),
            "mcp-server" => Some(AiFramework::McpServer),
            _ => None,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
