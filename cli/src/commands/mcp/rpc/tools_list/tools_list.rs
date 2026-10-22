//! List the existing native MagiCore tool schemas.
//! Trả danh sách schema tool MagiCore hiện có.

use crate::commands::mcp::{JsonRpcRequest, JsonRpcResponse};
use serde_json::json;

pub(in crate::commands::mcp) fn tools_list(req: &JsonRpcRequest) -> JsonRpcResponse {
    JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        id: req.id.clone(),
        result: Some(json!({
            "tools": [
                {
                    "name": "mgc_install",
                    "description": "Execute MagiCore package installation with CAS reflink/hardlink caching",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "packages": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "Optional list of package specs (name@version) to install"
                            },
                            "frozen": {
                                "type": "boolean",
                                "description": "Fail if lockfile needs update (CI mode)"
                            }
                        }
                    }
                },
                {
                    "name": "mgc_add",
                    "description": "Add dependencies to the current polyglot project",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "packages": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "List of packages to add"
                            },
                            "dev": {
                                "type": "boolean",
                                "description": "Add as devDependency"
                            },
                            "compat_runtime": {
                                "type": "string",
                                "description": "Explicit toolchain opt-in for delegated lanes (e.g. pip, cargo, go). Native lanes ignore it; delegated lanes FAIL CLOSED without it."
                            }
                        },
                        "required": ["packages"]
                    }
                },
                {
                    "name": "mgc_audit",
                    "description": "Run supply-chain security audit and 24-hour release quarantine checks",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "fix": {
                                "type": "boolean",
                                "description": "Automatically bump vulnerable packages if safe"
                            }
                        }
                    }
                },
                {
                    "name": "mgc_workspace_info",
                    "description": "Get monorepo topology, computation build cache and catalog mappings",
                    "inputSchema": {
                        "type": "object",
                        "properties": {}
                    }
                },
                {
                    "name": "mgc_uninstall",
                    "description": "Remove dependencies through the detected core's ownership gate",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "packages": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "Package names to remove"
                            },
                            "compat_runtime": {
                                "type": "string",
                                "description": "Explicit toolchain opt-in for delegated lanes"
                            }
                        },
                        "required": ["packages"]
                    }
                },
                {
                    "name": "mgc_versions",
                    "description": "List published versions and dist-tags for a registry package",
                    "inputSchema": {
                        "type": "object",
                        "properties": {
                            "package": { "type": "string" }
                        },
                        "required": ["package"]
                    }
                },
                {
                    "name": "mgc_outdated",
                    "description": "List outdated dependencies declared in the current project",
                    "inputSchema": { "type": "object", "properties": {} }
                },
                {
                    "name": "mgc_config",
                    "description": "Read local configuration with sensitive values redacted",
                    "inputSchema": { "type": "object", "properties": {} }
                }
            ]
        })),
        error: None,
    }
}
