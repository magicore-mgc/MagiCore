//! Wires converts v1 records without fabricating package integrity.
//! Nối owner: Chuyển record v1 mà không bịa integrity package.

mod v1_to_v2;
pub use self::v1_to_v2::migrate_v1_to_v2;
#[cfg(test)]
mod test;
