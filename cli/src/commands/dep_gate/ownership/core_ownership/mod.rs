//! Per-core ownership dispatch wiring — wiring dispatch ownership theo core.
//! Module này chỉ khai báo owner con và xuất API; không chứa logic nghiệp vụ.
mod ai;
mod app;
mod cicd;
mod cloud;
mod core_ownership;
mod game;
mod hardware;
mod iot;
mod lib;
mod web;

pub(in crate::commands::dep_gate) use core_ownership::owner_for_core;
