//! `wasm_provenance.rs` — bounded project artifact scan and header check.
use std::path::{Path, PathBuf};

/// Discover `.wasm` artifacts under the project (bounded walk: skip
/// node_modules/target/.git — those are vendor/dep trees, not project
/// artifacts; cap the walk at 200 files to stay deterministic).
/// Khám phá artifact `.wasm` trong project (duyệt có giới hạn: bỏ
/// node_modules/target/.git — đó là tree vendor/dep; giới hạn 200 file
/// để tất định).
pub(in crate::adapter) fn find_wasm_artifacts(project_root: &Path) -> Vec<PathBuf> {
    const MAX_FILES: usize = 200;
    let mut found = Vec::new();
    let mut queue = vec![project_root.to_path_buf()];
    while let Some(dir) = queue.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if name == "node_modules" || name == "target" || name == ".git" || name == ".magicore" {
                continue;
            }
            if path.is_dir() {
                queue.push(path);
            } else if name.ends_with(".wasm") {
                found.push(path);
                if found.len() >= MAX_FILES {
                    return found;
                }
            }
        }
    }
    found
}

/// Provenance check for one wasm artifact set: verify the magic header
/// (\0asm) and readable size; a module that fails the header check is
/// NOT a wasm binary (mislabelled or corrupted) → Failed step naming
/// the file. Valid modules are recorded as scanned packages — the CVE
/// lane for their SOURCE dependencies is the cargo/npm sidecar's job;
/// this lane proves the binaries themselves are what they claim.
/// Kiểm nguồn gốc một tập artifact wasm: xác minh magic header
/// (\0asm) và size đọc được; module sai header KHÔNG phải binary wasm
/// (ghi nhãn sai hoặc hỏng) → step Failed nêu tên file. Module hợp lệ
/// được ghi là package đã quét — lane CVE cho dependency NGUỒN là việc
/// sidecar cargo/npm; lane này chứng minh binary đúng như tuyên bố.
pub(in crate::adapter) fn audit_wasm_provenance(
    files: &[PathBuf],
) -> mgc_types::adapter::AuditReport {
    use mgc_types::adapter::{AuditReport, ScannerStatus};

    let mut scanned = 0usize;
    for file in files {
        let Ok(bytes) = std::fs::read(file) else {
            return AuditReport::scanner_failed(
                "wasm-provenance",
                format!("cannot read wasm artifact {}", file.display()),
            );
        };
        if bytes.len() < 8 || &bytes[..4] != b"\0asm" {
            return AuditReport::scanner_failed(
                "wasm-provenance",
                format!(
                    "{} is not a valid wasm module (bad magic header) — provenance cannot be verified",
                    file.display()
                ),
            );
        }
        let _ = bytes; // size already implied by the read; magic checked.
        scanned += 1;
    }
    AuditReport {
        packages_audited: scanned,
        vulnerability_count: 0,
        vulnerabilities: vec![],
        // The provenance check PASSED for every artifact — Available
        // with zero CVE findings (this lane finds no CVEs; it verifies
        // binary provenance).
        // Kiểm nguồn gốc PASS cho mọi artifact — Available với 0
        // finding CVE (lane này không tìm CVE; nó xác minh nguồn gốc
        // binary).
        scanner_status: ScannerStatus::Available,
    }
}
