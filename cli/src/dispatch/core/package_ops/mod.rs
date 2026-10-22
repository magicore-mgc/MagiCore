//! Package lifecycle dispatch facade. / Facade điều phối vòng đời package.

mod ai;
mod app;
mod cicd;
mod clo;
mod game;
mod hardware;
mod iot;
mod lib;
mod package_ops;
mod web;

pub(in crate::dispatch) use package_ops::dispatch;
