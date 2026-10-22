//! Terraform checksum presence rule route.
//! Route rule nhận diện checksum presence của Terraform.

mod checksum_presence;

pub(super) use checksum_presence::observe;
