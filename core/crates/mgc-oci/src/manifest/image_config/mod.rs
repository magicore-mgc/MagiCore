//! OCI image configuration models.
//! Model cấu hình OCI image.

mod image_config;
pub use image_config::{OciConfig, OciHistory, OciImageConfig, OciRootFs};

#[cfg(test)]
mod test;
