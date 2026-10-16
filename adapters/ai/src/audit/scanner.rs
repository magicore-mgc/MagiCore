//! Security scanners cho model files.

use super::Finding;
use mgc_types::MgResult;
use std::io::Read;
use std::path::Path;

/// Scan pickle file for dangerous imports
/// P1: Byte pattern matching (simple but effective for common attacks)
/// P2 (deferred): Deep bytecode AST parsing with pyo3 when real exploit case appears
/// This is a heuristic, not a complete pickle verifier. Matching signatures
/// fail closed for known patterns; absence of a match does not prove safety.
/// Deep parsing would add a Python dependency and a larger scanner attack surface.
pub fn scan_pickle(path: &Path) -> MgResult<Vec<Finding>> {
    const CHUNK_SIZE: usize = 64 * 1024;
    let mut findings = Vec::new();

    // Check for common malicious pickle patterns (opcodes + imports)
    // Pickle opcodes: GLOBAL (c), REDUCE (R), BUILD (\x81), INST (i)
    let dangerous_patterns: &[(&[u8], &str)] = &[
        (b"os.system", "os.system (arbitrary command execution)"),
        (b"subprocess", "subprocess (shell access)"),
        (b"eval", "eval (code injection)"),
        (b"exec", "exec (code execution)"),
        (b"__import__", "__import__ (dynamic import)"),
        (b"socket", "socket (network access)"),
        (b"builtins", "builtins (unrestricted access)"),
        // Pickle opcodes for code execution
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

    // Check file size (pickle > 10GB suspicious).
    // Kiểm kích thước file (pickle > 10GB cần xem xét).
    if metadata.len() > 10 * 1024 * 1024 * 1024 {
        findings.push(
            Finding::high("size", "Pickle file unusually large (>10GB)")
                .with_file(&path.to_string_lossy()),
        );
    }

    Ok(findings)
}

/// Scan safetensors for metadata issues
pub fn scan_safetensors(path: &Path) -> MgResult<Vec<Finding>> {
    const MAX_HEADER_BYTES: u64 = 100 * 1024 * 1024;
    let mut findings = Vec::new();
    let file_size = std::fs::metadata(path)?.len();

    // SafeTensors format: 8-byte header (JSON length) + JSON metadata + tensors
    if file_size < 8 {
        findings.push(
            Finding::critical("safetensors-format", "Invalid safetensors header")
                .with_file(&path.to_string_lossy()),
        );
        return Ok(findings);
    }

    // Read header length (little-endian u64) with checked arithmetic.
    // Đọc độ dài header little-endian u64 bằng phép tính có kiểm tra.
    let mut file = std::fs::File::open(path)?;
    let mut length_bytes = [0u8; 8];
    file.read_exact(&mut length_bytes)?;
    let header_len = u64::from_le_bytes(length_bytes);

    if header_len > MAX_HEADER_BYTES {
        findings.push(
            Finding::high("safetensors-header", "Suspiciously large metadata (>100MB)")
                .with_file(&path.to_string_lossy()),
        );
        return Ok(findings);
    }

    let Some(header_end) = header_len.checked_add(8) else {
        findings.push(
            Finding::critical("safetensors-truncated", "Truncated or corrupted file")
                .with_file(&path.to_string_lossy()),
        );
        return Ok(findings);
    };
    if header_end > file_size {
        findings.push(
            Finding::critical("safetensors-truncated", "Truncated or corrupted file")
                .with_file(&path.to_string_lossy()),
        );
        return Ok(findings);
    }

    Ok(findings)
}

/// Scan weight files for suspicious patterns
pub fn scan_weights(path: &Path) -> MgResult<Vec<Finding>> {
    let metadata = std::fs::metadata(path)?;
    let mut findings = Vec::new();

    // Check file size
    let size_gb = metadata.len() / (1024 * 1024 * 1024);

    if size_gb > 100 {
        findings.push(
            Finding::medium("size", &format!("Very large model file ({}GB)", size_gb))
                .with_file(&path.to_string_lossy()),
        );
    }

    // For PyTorch .pt/.pth/.bin checkpoints, scan pickle signatures.
    // (Với checkpoint PyTorch .pt/.pth/.bin, quét các pickle signature.)
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Some(ext) = path.extension().and_then(|ext| ext.to_str())
        && ["pt", "pth", "bin"]
            .iter()
            .any(|candidate| ext.eq_ignore_ascii_case(candidate))
    {
        // PyTorch checkpoints can use different pickle protocols and payloads.
        // Scan the payload signatures instead of requiring one exact prefix.
        // (Checkpoint có thể dùng nhiều pickle protocol; quét signature thay vì ép prefix.)
        findings.extend(scan_pickle(path)?);
    }

    Ok(findings)
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

#[cfg(test)]
#[path = "test/scanner_tests.rs"]
mod tests;
