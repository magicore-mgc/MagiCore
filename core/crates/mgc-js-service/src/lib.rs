//! `mgc-js-service` — shared JS/npm service seed (S07; D014/A05).
//!
//! Responsibility: own the core-independent npm/JS plumbing used by the Web
//! core today and by the App and Cloud cores after the S08 migration —
//! `package.json` manifest IO (`manifest`), npm registry endpoint policy
//! (`registry`), and the pure npm Bulk Advisory pipeline primitives
//! (`audit`). One owner for the Node service (A05); no adapter-to-adapter
//! edges (R07).
//! Trách nhiệm: sở hữu plumbing npm/JS độc lập core mà core Web dùng hôm nay
//! và core App/Cloud dùng sau migration S08 — IO manifest `package.json`
//! (`manifest`), policy endpoint registry npm (`registry`), và primitive thuần
//! của pipeline npm Bulk Advisory (`audit`). Một owner cho service Node (A05);
//! không cạnh adapter-to-adapter (R07).
//!
//! Owner / Chủ sở hữu: S07 slice (`wholeprojectrefactor` D014, A05, P03).
//! Callers / Bên gọi: `mgc-web-adapter` compat facades (S07; sole caller until
//! S08); blessed-interim external users reach these symbols through the web
//! facades only (`adapters/app` RN manifest wrappers + audit scanner — S06
//! §3/D014; retarget to this crate in S08). No other callers.
//! Excluded behavior / Hành vi loại trừ (stays in web until S08 at least):
//! the `run_audit` runner + `run_audit_fix` + v4/legacy lockfile pin sourcing
//! (`v4_pins_for_audit`, legacy reader, rival-lockfile gate) stay in
//! `mgc-web-adapter::audit` — they read the web-owned lockfile seam
//! (`crate::lockfile`) and cannot move verbatim without duplicating lock
//! ownership (R07/R13). `WebAdapter` construction and the resolve/install
//! pipeline stay in web (engine migration is S08+, removed last in S10).
//! The `DEFAULT_NPM_REGISTRY` value is a compatibility surface (R09/R14):
//! moved, never changed — note the pre-existing same-valued duplicate in
//! `mgc-config::chain` (out of scope for S07; dedup is a central-edit task).

// Test fixtures use unwrap for concise setup failures; production code keeps the workspace lint.
#![cfg_attr(test, allow(clippy::unwrap_used))]

pub mod audit;
pub mod error;
pub mod manifest;
pub mod registry;
pub mod resolve;
