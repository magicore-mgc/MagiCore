//! Errors specific to store-index consistency checks.
//! Lỗi riêng cho kiểm tra nhất quán của store index.

#[derive(Debug)]
pub(in crate::index) enum IndexError {
    EmptySqliteDuringPrune,
}

impl std::fmt::Display for IndexError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySqliteDuringPrune => formatter.write_str(
                "index loaded from msgpack but SQLite is empty — aborting prune, run rebuild()",
            ),
        }
    }
}

impl std::error::Error for IndexError {}
