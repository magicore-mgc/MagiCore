//! Terraform-specific parser error route.
//! Route lỗi parser riêng của Terraform.

mod error;

pub(super) use error::TerraformLockError;
