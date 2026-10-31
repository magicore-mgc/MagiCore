//! `audit/mod.rs` — Unified security audit pipeline for ALL cores.
//!
//! One contract, one pipeline (Tech Lead 2026-09-09 §1):
//! detect core → adapter → scanner → typed report → policy → exit code.
//! The CLI only dispatches and renders; the adapters run the scanners and
//! the shared finisher enforces the fail-closed exit contract.
//!
//! P1-B (2026-09-09): `--format json|sarif|cyclonedx` prints ONLY the
//! machine payload (no table) while keeping the SAME exit contract —
//! CI can ingest evidence without parsing terminal art.
//!
//! Pipeline audit THỐNG NHẤT cho mọi core: CLI chỉ dispatch + render;
//! adapter chạy scanner; finisher chung giữ exit contract fail-closed
//! (Available → exit theo finding; Unavailable → warning, strict thì
//! fail). `--format` in đúng payload máy, giữ nguyên exit contract.

pub mod ai;
pub mod app;
pub mod cicd;
pub mod clo;
pub mod game;
pub mod hardware;
pub mod iot;
pub mod lib;
pub mod signatures;
pub mod web;

mod audit;
mod output_format;
mod report;
mod strict_mode;

pub use audit::run;
pub(crate) use audit::run_strict;
pub(super) use audit::{finish_and_print, run_adapter_audit};
pub(crate) use output_format::OutputFormat;
pub(crate) use report::print_audit_report;
#[cfg(test)]
pub(crate) use report::{advisory_id_label, finding_table_labels, is_cve_id};
pub(crate) use strict_mode::{StrictMode, enforce_audit_exit};

#[cfg(test)]
#[path = "test/audit_format_test.rs"]
mod tests;
