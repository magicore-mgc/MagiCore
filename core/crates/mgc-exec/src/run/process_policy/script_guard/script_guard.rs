//! Script-file shebang policy guard.
//! Guard policy shebang của file script.

use super::super::{error::ProcessPolicyError, forbidden_process_name};

/// Reject a script whose first shebang line selects a forbidden package manager.
/// Từ chối script nếu dòng shebang đầu tiên chọn package manager bị cấm.
pub(in crate::run::implementation) fn reject_forbidden_script_file(
    command: &str,
) -> anyhow::Result<()> {
    let path = std::path::Path::new(command);
    if !path.is_file() {
        return Ok(());
    }

    let Ok(bytes) = std::fs::read(path) else {
        return Ok(());
    };
    if !bytes.starts_with(b"#!") {
        return Ok(());
    }

    // Inspect only the interpreter declaration; later mentions can be benign.
    // Chỉ kiểm tra khai báo interpreter; nội dung phía sau có thể hợp lệ.
    // Runtime process guards still block forbidden package-manager execution.
    // Guard process runtime vẫn chặn việc thực thi package manager bị cấm.
    let shebang = bytes
        .split(|byte| *byte == b'\n')
        .next()
        .unwrap_or_default();
    let shebang = String::from_utf8_lossy(shebang);
    if let Some(tool) = forbidden_process_name("", &shebang, &[]) {
        return Err(anyhow::Error::new(
            ProcessPolicyError::ForbiddenScriptInterpreter {
                path: path.display().to_string(),
                tool,
            },
        ));
    }
    Ok(())
}
