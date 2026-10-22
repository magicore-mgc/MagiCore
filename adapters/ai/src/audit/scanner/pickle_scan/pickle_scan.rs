//! Scan model files for known dangerous pickle byte signatures.
//! Quét file model để tìm signature byte pickle nguy hiểm đã biết.

use super::super::byte_match::contains_bytes;
use crate::audit::Finding;
use mgc_types::MgResult;
use std::io::Read;
use std::path::Path;

/// Scan pickle data for known dangerous signatures; this remains a heuristic.
/// Quét signature pickle nguy hiểm đã biết; đây vẫn là phương pháp heuristic.
/// P1 uses bounded-memory byte matching; deep bytecode parsing is deferred.
/// P1 so khớp byte với bộ nhớ giới hạn; phân tích bytecode sâu được để sau.
/// Missing signatures do not prove the artifact safe.
/// Không tìm thấy signature không chứng minh artifact an toàn.
pub fn scan_pickle(path: &Path) -> MgResult<Vec<Finding>> {
    const CHUNK_SIZE: usize = 64 * 1024;
    let mut findings = Vec::new();

    // Match common malicious pickle patterns and opcode/import signatures.
    // Tìm pattern pickle độc hại cùng signature opcode/import thường gặp.
    let dangerous_patterns: &[(&[u8], &str)] = &[
        (b"os.system", "os.system (arbitrary command execution)"),
        (b"subprocess", "subprocess (shell access)"),
        (b"eval", "eval (code injection)"),
        (b"exec", "exec (code execution)"),
        (b"__import__", "__import__ (dynamic import)"),
        (b"socket", "socket (network access)"),
        (b"builtins", "builtins (unrestricted access)"),
        // Pickle opcodes for code execution.
        // Opcode pickle có thể dẫn đến thực thi mã.
        (b"c__builtin__\neval\n", "pickle GLOBAL opcode with eval"),
        (b"cos\nsystem\n", "pickle GLOBAL opcode with os.system"),
        (b"\x81", "pickle BUILD opcode (class instantiation)"),
    ];

    let metadata = std::fs::metadata(path)?;
    let mut reader = std::fs::File::open(path)?;
    let max_pattern_len = dangerous_patterns
        .iter()
        .map(|(pattern, _)| pattern.len())
        .max()
        .unwrap_or(1);
    let mut pending = Vec::with_capacity(CHUNK_SIZE + max_pattern_len);
    let mut chunk = [0u8; CHUNK_SIZE];
    let mut found = vec![false; dangerous_patterns.len()];

    loop {
        let bytes_read = reader.read(&mut chunk)?;
        if bytes_read == 0 {
            break;
        }
        pending.extend_from_slice(&chunk[..bytes_read]);
        for (index, (pattern, _)) in dangerous_patterns.iter().enumerate() {
            if !found[index] && contains_bytes(&pending, pattern) {
                found[index] = true;
            }
        }
        if found.iter().all(|found| *found) {
            break;
        }
        let keep_from = pending
            .len()
            .saturating_sub(max_pattern_len.saturating_sub(1));
        pending.drain(..keep_from);
    }

    for (found, (_, desc)) in found.into_iter().zip(dangerous_patterns) {
        if found {
            findings.push(
                Finding::critical(
                    "pickle-exploit",
                    &format!("Dangerous pattern detected: {}", desc),
                )
                .with_file(&path.to_string_lossy()),
            );
        }
    }

    // Flag pickle artifacts larger than the existing suspicious-file threshold.
    // Đánh dấu artifact pickle vượt ngưỡng kích thước đáng ngờ hiện tại.
    if metadata.len() > 10 * 1024 * 1024 * 1024 {
        findings.push(
            Finding::high("size", "Pickle file unusually large (>10GB)")
                .with_file(&path.to_string_lossy()),
        );
    }

    Ok(findings)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
