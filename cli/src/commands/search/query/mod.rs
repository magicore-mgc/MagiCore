//! Wire search query construction and its nearest tests.
//! Nối logic tạo truy vấn tìm kiếm với test gần nhất.

#[path = "query.rs"]
mod query;

pub(super) use query::{SEARCH_PAGE_SIZE, SearchRequest};
