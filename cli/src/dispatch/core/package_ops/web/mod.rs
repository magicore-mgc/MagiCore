//! web package dispatch facade. / Facade dispatch package của core web.

#[cfg(feature = "web")]
#[path = "web.rs"]
mod web;

#[cfg(feature = "web")]
pub(in crate::dispatch) use web::{dispatch, matches};

#[cfg(not(feature = "web"))]
pub(in crate::dispatch) fn matches(_command: &crate::dispatch::types::CoreCommand) -> bool {
    false
}

#[cfg(not(feature = "web"))]
pub(in crate::dispatch) async fn dispatch(
    _command: crate::dispatch::types::CoreCommand,
) -> anyhow::Result<()> {
    Err(crate::error::core_not_in_build("web"))
}
