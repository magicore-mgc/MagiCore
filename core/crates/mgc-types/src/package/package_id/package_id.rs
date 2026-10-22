//! Defines package identities and their text representation.
//! Định nghĩa identity package và biểu diễn dạng text.

use super::super::PackageName;
use crate::error::{MgError, MgResult};
use crate::version::Version;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct PackageId {
    name: PackageName,
    version: Version,
}

impl PackageId {
    pub fn new(name: PackageName, version: Version) -> Self {
        Self { name, version }
    }

    pub fn parse(input: &str) -> MgResult<Self> {
        let Some(at) = input.rfind('@') else {
            return Err(MgError::InvalidPackageSpec(input.to_string()));
        };
        let name = PackageName::new(&input[..at])?;
        let version = Version::parse(&input[at + 1..])?;
        Ok(Self::new(name, version))
    }

    pub fn name(&self) -> &PackageName {
        &self.name
    }

    pub fn name_str(&self) -> &str {
        self.name.as_str()
    }

    pub fn version(&self) -> &Version {
        &self.version
    }
}

impl fmt::Display for PackageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}@{}", self.name, self.version)
    }
}
