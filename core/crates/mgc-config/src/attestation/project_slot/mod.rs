//! project_slot owner wiring.
//! Wiring owner Ánh xạ project sang identity slot ngoài project và đọc bytes cấu hình an toàn.
mod project_slot;
pub use project_slot::{ATTESTATION_FILE, attestation_dir, read_config_bytes};
