//! CI/CD adapter type retaining its public provider field.
//! Kiểu adapter CICD giữ nguyên field provider công khai.

use crate::provider::CicdProvider;

pub struct CicdAdapter {
    pub provider: CicdProvider,
}

#[cfg(test)]
#[path = "adapter/test/mod.rs"]
mod tests;
