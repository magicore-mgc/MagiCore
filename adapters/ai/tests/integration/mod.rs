//! AI integration test owner exports — export owner test modules.
//! This file only wires focused behavior owners. — File này chỉ nối các owner hành vi.

#[path = "adapter_contract/adapter_contract.rs"]
pub(crate) mod adapter_contract;
#[path = "audit_sbom/audit_sbom.rs"]
pub(crate) mod audit_sbom;
#[path = "project_detection/project_detection.rs"]
pub(crate) mod project_detection;
#[path = "python_lane/python_lane.rs"]
pub(crate) mod python_lane;
#[path = "support/mod.rs"]
pub(crate) mod support;
