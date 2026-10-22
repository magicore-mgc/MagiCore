//! CI/CD manifest classification module wiring.
//! Wiring module phân loại manifest CI/CD.

mod manifest_detection;

pub(crate) use manifest_detection::manifest_is_cicd;
