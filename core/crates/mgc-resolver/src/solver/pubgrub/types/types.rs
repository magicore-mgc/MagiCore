use crate::version::VersionSet;
use mgc_types::{PackageName, Version};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Term {
    Positive(PackageName, VersionSet),
    Negative(PackageName, VersionSet),
}

#[derive(Debug, Clone)]
pub struct Incompatibility {
    pub terms: Vec<Term>,
    pub cause: Option<Cause>,
}

impl Incompatibility {
    pub fn new(terms: Vec<Term>) -> Self {
        Self { terms, cause: None }
    }

    pub fn with_cause(mut self, cause: Cause) -> Self {
        self.cause = Some(cause);
        self
    }
}

#[derive(Debug, Clone)]
pub enum Cause {
    Dependency {
        package: PackageName,
        version: Version,
        dep_package: PackageName,
        dep_version_set: Box<VersionSet>,
    },
    Conflict {
        incompatibility1: Box<Incompatibility>,
        incompatibility2: Box<Incompatibility>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    Assigned(PackageName, Version, Box<VersionSet>),
    Backtracked(PackageName),
}

#[derive(Debug, Clone)]
pub enum DerivationTree {
    Root(Vec<DerivationTree>),
    Dependency {
        package: PackageName,
        version: Version,
        dep: Box<DerivationTree>,
    },
    Conflict {
        left: Box<DerivationTree>,
        right: Box<DerivationTree>,
    },
    NoVersion {
        package: PackageName,
        constraint: VersionSet,
    },
}

impl std::fmt::Display for DerivationTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DerivationTree::Root(children) => {
                for child in children {
                    writeln!(f, "  {child}")?;
                }
                Ok(())
            }
            DerivationTree::Dependency {
                package,
                version,
                dep,
            } => {
                write!(f, "{package}@{version} depends on {dep}")
            }
            DerivationTree::Conflict { left, right } => {
                write!(f, "conflict: {left} vs {right}")
            }
            DerivationTree::NoVersion {
                package,
                constraint,
            } => {
                write!(f, "no version of {package} satisfies {constraint}")
            }
        }
    }
}
