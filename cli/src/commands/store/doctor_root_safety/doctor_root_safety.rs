//! Checks CAS-root symlink ancestry and real-directory safety.
//! Kiểm tra symlink ancestor và độ an toàn thư mục CAS root.

use std::path::Path;

pub(in crate::commands::store) fn inspect_cas_root_safety(
    cas_root: &Path,
    json: bool,
    report: &mut super::super::doctor_report::DoctorReport,
) -> bool {
    let mut hard_failure = false;
    // Pillar 2 — CAS root safety (P0-A/P0-B: symlink checks must hold at
    // doctor time too — a root swapped to a symlink between installs is
    // a live attack window, not a stale artifact).
    // (Trụ 2 — an toàn root CAS (P0-A/P0-B: check symlink phải còn đúng
    // lúc doctor — root bị đổi thành symlink giữa 2 lần install là cửa
    // sổ tấn công sống, không phải tàn dư cũ).)
    if cas_root.exists() {
        match mgc_store::cas::validate_cas_root(cas_root) {
            Ok(()) => {
                report.cas_root_ok = true;
                if !json {
                    println!("  [ok] CAS root is a real directory (no symlink ancestry)");
                }
            }
            Err(e) => {
                hard_failure = true;
                if !json {
                    println!("  [FAIL] CAS root unsafe: {e}");
                    println!(
                        "  repair hint: move the store to a real directory and \
                         remove the symlink chain — refusing to operate through \
                         a redirected root"
                    );
                }
            }
        }
    } else {
        report.cas_root_ok = true;
        if !json {
            println!("  [ok] no CAS directory yet (nothing to validate)");
        }
    }
    hard_failure
}
