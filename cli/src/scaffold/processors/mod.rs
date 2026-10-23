//! processors/<core>.rs — scaffold files per core (v5: `scaffold/ → processors/`).
//! Web scaffold giữ tại processor.rs (web templates/contract engine).

pub mod ai;
pub mod app;
pub mod cicd;
pub mod clo;
pub mod game;
pub mod hardware;
pub mod iot;
pub mod lib;
pub mod web;

mod common;

pub(crate) use common::{slugify, write_file};
