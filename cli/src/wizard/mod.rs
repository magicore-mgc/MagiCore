//! Interactive wizards, one owner folder per core.
//! Wizard tương tác, mỗi core một owner folder.

pub mod ai;
pub mod app;
pub mod cicd;
pub mod cloud;
pub mod engine;
pub mod game;
pub mod hardware;
#[cfg(feature = "iot")]
pub mod iot;
pub mod lib;
pub mod web;
