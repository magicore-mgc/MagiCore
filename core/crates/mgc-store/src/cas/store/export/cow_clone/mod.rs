//! Cow Clone owner wiring.
//! Nối owner cow clone.

mod cow_clone;

pub(in crate::cas::store) use self::cow_clone::try_cow_clone;
pub(in crate::cas::store::export) use self::cow_clone::try_cow_clone_from_file;

#[cfg(test)]
mod test;
