//! Template name normalization API wiring.
//! Wiring API chuẩn hóa tên template.

mod normalize_name;

pub(in crate::scaffold::spec) use normalize_name::normalize_template_name;
