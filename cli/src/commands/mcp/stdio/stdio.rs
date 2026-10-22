//! Serve the newline-delimited JSON-RPC stdio session.
//! Phục vụ phiên stdio JSON-RPC theo từng dòng.

use super::{McpProtocolOutput, McpStdoutCapture, append_captured_stdout};
use crate::commands::mcp::{JsonRpcRequest, handle_rpc_request};
use anyhow::Result;
use std::io::{self, BufRead, Write};

pub(in crate::commands::mcp) async fn serve() -> Result<()> {
    let mut stdout = McpProtocolOutput::redirect_cli_output()?;
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            line.clear();
            continue;
        }

        if let Ok(req) = serde_json::from_str::<JsonRpcRequest>(trimmed) {
            let capture = McpStdoutCapture::begin()?;
            let mut response = handle_rpc_request(&req).await;
            let command_output = capture.finish()?;
            if !command_output.trim().is_empty() {
                append_captured_stdout(&mut response, command_output.trim_end());
            }
            let res_bytes = serde_json::to_vec(&response)?;
            stdout.write_all(&res_bytes)?;
            stdout.write_all(b"\n")?;
            stdout.flush()?;
        }

        line.clear();
    }

    Ok(())
}
