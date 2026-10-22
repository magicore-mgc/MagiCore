use anyhow::{Result, anyhow};

use crate::dispatch::types::CoreCommand;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PackageCore {
    Web,
    Game,
    Ai,
    Clo,
    Cicd,
    Iot,
    App,
    Lib,
    Hardware,
}

pub(super) fn owner_for(command: &CoreCommand) -> Option<PackageCore> {
    if super::web::matches(command) {
        return Some(PackageCore::Web);
    }
    if super::game::matches(command) {
        return Some(PackageCore::Game);
    }
    if super::ai::matches(command) {
        return Some(PackageCore::Ai);
    }
    if super::clo::matches(command) {
        return Some(PackageCore::Clo);
    }
    if super::cicd::matches(command) {
        return Some(PackageCore::Cicd);
    }
    if super::iot::matches(command) {
        return Some(PackageCore::Iot);
    }
    if super::app::matches(command) {
        return Some(PackageCore::App);
    }
    if super::lib::matches(command) {
        return Some(PackageCore::Lib);
    }
    if super::hardware::matches(command) {
        return Some(PackageCore::Hardware);
    }
    None
}

pub(in crate::dispatch) async fn dispatch(command: CoreCommand) -> Result<()> {
    match owner_for(&command) {
        Some(PackageCore::Web) => super::web::dispatch(command).await,
        Some(PackageCore::Game) => super::game::dispatch(command).await,
        Some(PackageCore::Ai) => super::ai::dispatch(command).await,
        Some(PackageCore::Clo) => super::clo::dispatch(command).await,
        Some(PackageCore::Cicd) => super::cicd::dispatch(command).await,
        Some(PackageCore::Iot) => super::iot::dispatch(command).await,
        Some(PackageCore::App) => super::app::dispatch(command).await,
        Some(PackageCore::Lib) => super::lib::dispatch(command).await,
        Some(PackageCore::Hardware) => super::hardware::dispatch(command).await,
        None => Err(anyhow!(
            "package dispatcher received a non-package CoreCommand"
        )),
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
