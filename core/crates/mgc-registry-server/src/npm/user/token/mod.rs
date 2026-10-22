//! Export admin-only npm token revocation.
//! Xuất chức năng thu hồi token npm chỉ dành cho admin.

mod token;
pub(in crate::npm) use token::revoke_token;
