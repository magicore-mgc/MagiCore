//! Shared Web and native package freshness command facade.
//! Façade lệnh kiểm tra phiên bản package dùng chung cho Web và core native.

mod api;
mod error;
mod model;
mod native;
mod presentation;
mod query;
#[cfg(feature = "web")]
mod web;

pub use api::{outdated_json, run};
