use crate::policy::tool_basename;

/// MGC-synthesized probe answers (Spec rev4 §5): only probes whose semantics
/// are exactly defined may be answered without spawning anything. Today that
/// is exactly one — `npm --version` → `"0.0.0"`, mirroring the PATH deny shim
/// (a compat behavior under review, not a general pattern).
/// (Probe do MGC tự trả lời (Spec rev4 §5): chỉ probe có semantics xác định
/// chính xác mới được trả lời mà không spawn. Hiện đúng một — `npm --version`
/// → `"0.0.0"`, mirror PATH deny shim (compat behavior đang review, không
/// phải mẫu chung).)
pub fn self_answer_for(program: &str, args: &[String]) -> Option<String> {
    if tool_basename(program) == "npm" && args.len() == 1 && args[0] == "--version" {
        return Some("0.0.0".to_string());
    }
    None
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
