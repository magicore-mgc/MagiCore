//! Wires extraction-root checks and their nearest test owner.
//! Nối kiểm tra root giải nén và owner test gần nhất.

mod extract_root;
pub(super) use self::extract_root::ensure_extract_root;
#[cfg(test)]
mod test;
