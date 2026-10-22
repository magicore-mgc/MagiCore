//! MessagePack index persistence exports.
//! Xuất thao tác lưu/truy xuất chỉ mục MessagePack.

mod persistence;
pub(super) use persistence::{load, packages_to_map, persist};
