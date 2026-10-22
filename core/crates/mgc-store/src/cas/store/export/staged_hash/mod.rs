//! Staged Hash owner wiring.
//! Nối owner staged hash.

mod staged_hash;

pub(in crate::cas::store::export) use self::staged_hash::hash_file;

#[cfg(test)]
mod test;
