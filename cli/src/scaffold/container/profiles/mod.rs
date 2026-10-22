//! Core-specific MGC-owned container lifecycle profiles.
//! Profile lifecycle container theo core, do MGC sở hữu.

mod ai;
mod app;
mod clo;
mod lib;
mod web;

pub(super) use ai::write_ai_docker_assets;
pub(super) use app::write_app_docker_assets;
pub(super) use clo::write_clo_docker_assets;
pub(super) use lib::write_lib_docker_assets;
pub(super) use web::{write_web_docker_assets, write_web_k8s_assets};
