//! Native Web Docker and local Kubernetes development profile.
//! Profile Docker Web native và Kubernetes development.

use crate::scaffold::container::error;
#[path = "web.rs"]
mod web;
pub(in crate::scaffold::container) use web::{write_web_docker_assets, write_web_k8s_assets};
