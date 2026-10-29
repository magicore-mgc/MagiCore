//! Stable App adapter facade and capability-owner wiring.
//! Facade ổn định cho App adapter và wiring owner capability.

mod app_adapter;
mod audit;
mod capability_fallbacks;
mod content_store;
mod dependency_resolver;
mod factory;
mod identity;
mod lifecycle;
mod lockfile;
mod optimizer_provider;
mod package_adapter;
mod pending_install_state;

pub use app_adapter::AppAdapter;
pub use factory::adapter_for;
pub(crate) use pending_install_state::PendingInstallState;
