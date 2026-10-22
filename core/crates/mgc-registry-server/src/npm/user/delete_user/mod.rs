//! Export admin-only account deletion.
//! Xuất chức năng xóa tài khoản chỉ dành cho admin.

mod delete_user;
pub(in crate::npm) use delete_user::delete_user;
