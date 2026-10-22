//! App integration test owner exports — export owner integration test App.
//! This file wires focused behavior modules. — File này chỉ nối các module hành vi.

#[path = "adapter_contract/adapter_contract.rs"]
pub(crate) mod adapter_contract;
#[path = "audit_sbom/audit_sbom.rs"]
pub(crate) mod audit_sbom;
#[path = "language_detection/language_detection.rs"]
pub(crate) mod language_detection;
#[path = "lifecycle/lifecycle.rs"]
pub(crate) mod lifecycle;
#[path = "lock_replay/lock_replay.rs"]
pub(crate) mod lock_replay;
#[path = "support/mod.rs"]
pub(crate) mod support;
