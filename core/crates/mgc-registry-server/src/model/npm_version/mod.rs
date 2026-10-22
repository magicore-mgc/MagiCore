//! npm package version and embedded metadata records.
//! Version package npm và các record metadata lồng nhau.

mod npm_version;

pub use npm_version::{
    Author, Bugs, DistInfo, PackageVersion, PublishConfig, Repository, Signature,
};
