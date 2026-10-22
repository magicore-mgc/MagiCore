//! Lib integration test owner exports — export owner integration test Lib.
//! This file wires only behavior modules. — File này chỉ nối module theo hành vi.

#[path = "audit_sbom/audit_sbom.rs"]
pub(crate) mod audit_sbom;
#[path = "capability_contract/capability_contract.rs"]
pub(crate) mod capability_contract;
#[path = "language_detection/language_detection.rs"]
pub(crate) mod language_detection;
#[path = "lifecycle/lifecycle.rs"]
pub(crate) mod lifecycle;
#[path = "manifest_io/manifest_io.rs"]
pub(crate) mod manifest_io;
#[path = "python_ownership/python_ownership.rs"]
pub(crate) mod python_ownership;
#[path = "support/mod.rs"]
pub(crate) mod support;
