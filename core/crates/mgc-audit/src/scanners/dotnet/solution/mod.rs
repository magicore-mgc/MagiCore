//! Wire solution discovery/parsing and its local test.
//! Nối tìm/parse solution và test cục bộ.

mod solution;
#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(super) use self::solution::find_solution_files;
pub use self::solution::read_solution_projects;
