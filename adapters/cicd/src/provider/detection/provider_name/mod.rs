//! CI/CD provider name parser module wiring.
//! Wiring module parser tên provider CI/CD.
//!
//! `provider_name.rs` re-exports the single canonical parser that the provider
//! type owns; nothing new is declared here.
//! (`provider_name.rs` xuất lại parser chuẩn duy nhất do provider type sở hữu;
//! ở đây không khai báo thêm gì.)

#[path = "provider_name.rs"]
mod provider_name;

pub(super) use provider_name::provider_from_str;
