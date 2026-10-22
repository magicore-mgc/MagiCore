//! Selects Web CAS or native-root checks and enforces repair policy.
//! Chọn kiểm tra Web CAS hoặc root native và áp chính sách repair.

use super::super::global_root_doctor::doctor_global_roots;
use super::super::project_ownership::{ensure_store_framework_owner, ensure_web_store_owner};
use super::super::route_selection::{StoreRoute, store_route};
use super::super::web_doctor::doctor;
use anyhow::Result;
use std::path::Path;

/// Dispatch health checking to the store route declared for the selected lane.
/// Điều phối health check theo store route của lane đã chọn.
pub(in crate::commands::store) fn run_store_doctor(
    project_root: &Path,
    repair: bool,
    core: &str,
    framework: Option<&str>,
    json: bool,
) -> Result<()> {
    let route = store_route(core, framework)
        .ok_or_else(|| crate::error::store_doctor_route_unsupported(core, framework))?;
    match route {
        StoreRoute::WebProject => {
            if core == "web" {
                if let Some(framework) = framework {
                    ensure_web_store_owner(project_root)?;
                    ensure_store_framework_owner(project_root, core, framework)?;
                } else {
                    ensure_web_store_owner(project_root)?;
                }
                let native = crate::commands::compat::CompatMode::Native;
                crate::commands::dep_gate::gate(
                    &crate::commands::dep_gate::DepContext::new(
                        "web",
                        Some(crate::commands::dep_gate::eco::JS),
                        None,
                        None,
                        crate::commands::dep_gate::DepOp::Gc,
                    ),
                    None,
                    &native,
                    None,
                )?;
            } else {
                let framework = framework
                    .ok_or_else(|| crate::error::store_doctor_route_unsupported(core, framework))?;
                ensure_store_framework_owner(project_root, core, framework)?;
                if repair {
                    return Err(crate::error::store_doctor_repair_unsupported(core));
                }
            }
            doctor(project_root, repair, core, framework, json)
        }
        StoreRoute::Global(roots) => {
            let framework = framework
                .ok_or_else(|| crate::error::store_doctor_route_unsupported(core, framework))?;
            ensure_store_framework_owner(project_root, core, framework)?;
            if repair {
                return Err(crate::error::store_doctor_repair_unsupported(core));
            }
            doctor_global_roots(core, framework, roots, json)
        }
    }
}
