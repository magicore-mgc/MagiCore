//! Aggregate audit findings for every recognized mobile-app manifest.
//! Tổng hợp finding audit từ mọi manifest app mobile đã nhận diện.

use super::{audit_cocoapods, audit_flutter, audit_kotlin, audit_swift};
use mgc_types::MgResult;
use mgc_types::adapter::AuditReport;
use std::path::Path;

pub async fn audit_multi(project_root: &Path) -> MgResult<AuditReport> {
    // Aggregate EVERY detected manifest — never first-match (Tech Lead
    // 2026-09-09: a multi-language project like React Native must merge
    // every ecosystem result into one report).
    // Tổng hợp MỌI manifest nhận diện — không first-match: project đa
    // ngôn ngữ (vd React Native) phải gộp kết quả mọi ecosystem.
    let mut plan = mgc_audit::AuditPlan::new();

    if project_root.join("pubspec.yaml").exists() {
        let root = project_root.to_path_buf();
        plan.add_step(mgc_audit::ScanStep {
            ecosystem: "flutter",
            scanner: "pub-osv",
            run: Box::new(move || {
                let root = root.clone();
                Box::pin(async move { audit_flutter(&root).await })
            }),
        });
    }

    // JS lane (P2 React Native aggregate 2026-09-10): mgc.lock /
    // bun.lock / deno.lock pins ride the npm Bulk Advisory flow — the
    // SAME pipeline as the web core. Without a JS lockfile the step is
    // absent (an un-manifested lane never fakes a clean step).
    // Lane JS (aggregate React Native): ghim mgc.lock / bun.lock /
    // deno.lock đi qua npm Bulk Advisory — CÙNG pipeline với core web.
    // Thiếu lockfile JS thì bỏ hẳn step (lane không manifest không bịa
    // step sạch).
    let js_locks_exist = project_root.join("mgc.lock").is_file()
        || project_root.join("bun.lock").is_file()
        || project_root.join("deno.lock").is_file();
    if js_locks_exist {
        let root = project_root.to_path_buf();
        plan.add_step(mgc_audit::ScanStep {
            ecosystem: "web/javascript",
            scanner: "npm-bulk-advisory",
            run: Box::new(move || {
                let root = root.clone();
                Box::pin(async move { audit_app_js_deps(&root).await })
            }),
        });
    }

    let has_gradle = project_root.join("build.gradle").exists()
        || project_root.join("build.gradle.kts").exists();
    if has_gradle {
        let root = project_root.to_path_buf();
        plan.add_step(mgc_audit::ScanStep {
            ecosystem: "kotlin",
            scanner: "owasp-dependency-check",
            run: Box::new(move || {
                let root = root.clone();
                Box::pin(async move { audit_kotlin(&root).await })
            }),
        });
    }

    // iOS lanes (P2 2026-09-10): SPM + CocoaPods — a React Native app
    // with Pods merges iOS findings into the same aggregate.
    // Lane iOS: SPM + CocoaPods — app React Native có Pods gộp finding
    // iOS vào cùng aggregate.
    // Gate parity (R2'): the gate uses the scanner's own locator — every
    // Package.resolved the scanner would find enters the plan.
    // Parity gate: gate dùng đúng locator của scanner — mọi
    // Package.resolved scanner tìm được đều vào plan.
    if mgc_audit::scanners::swift_resolved_path(project_root).is_some() {
        let root = project_root.to_path_buf();
        plan.add_step(mgc_audit::ScanStep {
            ecosystem: "swift",
            scanner: "osv-dev-api",
            run: Box::new(move || {
                let root = root.clone();
                Box::pin(async move { audit_swift(&root).await })
            }),
        });
    }
    if project_root.join("Podfile.lock").is_file() {
        let root = project_root.to_path_buf();
        plan.add_step(mgc_audit::ScanStep {
            ecosystem: "objc/cocoapods",
            scanner: "osv-dev-api",
            run: Box::new(move || {
                let root = root.clone();
                Box::pin(async move { audit_cocoapods(&root).await })
            }),
        });
    }

    if plan.is_empty() {
        return Ok(AuditReport::unsupported_ecosystem(
            "app/multi (no recognized app manifest found: pubspec.yaml, build.gradle(.kts), Package.resolved, Podfile.lock, mgc/bun/deno.lock)",
        ));
    }

    plan.execute().await
}

/// JS dependencies of an RN/multi app via the npm Bulk Advisory flow
/// (shared readers + the registry endpoint default). Runs the same
/// network path as the web core audit.
/// Dependency JS của app RN/multi qua npm Bulk Advisory (bộ đọc dùng
/// chung + endpoint registry mặc định) — cùng đường mạng với audit core
/// web.
async fn audit_app_js_deps(project_root: &Path) -> MgResult<AuditReport> {
    // Registry default owned by the shared JS service (S08/D014, R09 value
    // unchanged); `run_audit` itself stays web-owned (blessed-interim) —
    // it reads the web lockfile seam and was NOT moved in S07.
    // Hằng registry do service JS dùng chung sở hữu (giá trị không đổi);
    // `run_audit` vẫn của web (tạm chấp nhận) — đọc seam lockfile của web.
    mgc_web_adapter::audit::run_audit(project_root, mgc_js_service::registry::DEFAULT_NPM_REGISTRY)
        .await
}
