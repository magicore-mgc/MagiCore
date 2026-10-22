//! Wires safe destination-tree preparation.
//! Nối bước chuẩn bị cây đích an toàn.

mod cas_tree;
pub(super) use self::cas_tree::prepare_destination;
#[cfg(test)]
mod test;
