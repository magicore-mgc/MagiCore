//! Composer protocol owner test registration.
//! Đăng ký test cho owner protocol Composer.
//!
//! Subject files are registered from their nearest owner (composer.rs,
//! package_catalog/, version_constraints.rs) per RULE §5 nearest-owner;
//! `common` holds the captured Packagist fixture loader.
//! File test được đăng ký từ owner gần nhất; `common` giữ helper fixture.

pub(super) mod common;
