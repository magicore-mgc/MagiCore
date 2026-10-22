//! `audit.rs` — Web audit-provider aggregation / tổng hợp provider audit web.
use super::super::WebAdapter;
use super::wasm_provenance::{audit_wasm_provenance, find_wasm_artifacts};
use crate::audit::run_audit;
use async_trait::async_trait;
use mgc_types::{MgResult, adapter::AuditReport, capabilities::AuditProvider};
use std::path::Path;

#[async_trait]
impl AuditProvider for WebAdapter {
    /// Evidence: the multi-scanner audit aggregate (npm bulk advisory +
    /// rust/python/go sidecars + wasm provenance lane).
    /// Dẫn chứng: aggregate audit đa scanner (npm bulk advisory + sidecar
    /// rust/python/go + lane nguồn gốc wasm).
    fn probe_audit_provider(&self) -> MgResult<()> {
        Ok(())
    }

    async fn audit(&self, project_root: &Path) -> MgResult<AuditReport> {
        // Multi-language web aggregate (Tech Lead 2026-09-09 §2): the
        // npm graph is the primary scan; Rust/Python side manifests in
        // a web project (WASM crates, server scripts) join the same
        // aggregate — never silently skipped.
        // Aggregate đa ngôn ngữ cho web: graph npm là scan chính;
        // manifest Rust/Python kèm theo (crate WASM, script server) vào
        // cùng aggregate — không âm thầm bỏ qua.
        let mut plan = mgc_audit::AuditPlan::new();

        let root = project_root.to_path_buf();
        let registry = self.registry_url.clone();
        plan.add_step(mgc_audit::ScanStep {
            ecosystem: "web/javascript",
            scanner: "npm-bulk-advisory",
            run: Box::new(move || {
                let root = root.clone();
                let registry = registry.clone();
                Box::pin(async move { run_audit(&root, &registry).await })
            }),
        });

        // Sidecar lanes (R1): shared constructors — one detection rule
        // per manifest, the same rule every core uses. The python rule is
        // the broad shared one (requirements variants, pylock, uv.lock,
        // pyproject): a recognized-but-unscannable manifest surfaces as
        // the scanner's honest Failed, never a hidden skip.
        // Lane sidecar: constructor chung — một manifest một luật, mọi
        // core giống nhau.
        for step in [
            mgc_audit::rust_step(project_root),
            mgc_audit::python_step(project_root),
            mgc_audit::go_step(project_root),
            mgc_audit::java_step(project_root),
            mgc_audit::dotnet_step(project_root),
        ]
        .into_iter()
        .flatten()
        {
            plan.add_step(step);
        }

        // WASM provenance lane (P2 2026-09-10 matrix row "Web Rust/WASM"):
        // every .wasm artifact in the tree is checked — magic header,
        // size, and the name section — so an unknown/mislabelled binary
        // surfaces as a Failed step instead of riding along silently.
        // A wasm module produced by a DIFFERENT toolchain than the
        // Cargo sidecar cannot be attributed → Failed with the file
        // named (provenance is honest about what it cannot verify).
        // Lane nguồn gốc WASM: mọi artifact .wasm trong tree được kiểm
        // — magic header, kích thước, section name — để binary lạ/ghi
        // nhãn sai hiện thành step Failed thay vì đi ké âm thầm. Module
        // wasm do toolchain KHÁC sidecar Cargo sinh không quy được nguồn
        // → Failed kèm tên file (nguồn gốc trung thực về những gì
        // không kiểm chứng được).
        let wasm_files = find_wasm_artifacts(project_root);
        if !wasm_files.is_empty() {
            plan.add_step(mgc_audit::ScanStep {
                ecosystem: "wasm",
                scanner: "wasm-provenance",
                run: Box::new(move || {
                    let wasm_files = wasm_files.clone();
                    Box::pin(async move { Ok(audit_wasm_provenance(&wasm_files)) })
                }),
            });
        }

        plan.execute().await
    }
}
