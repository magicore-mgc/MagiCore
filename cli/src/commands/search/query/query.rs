//! Build the registry query and page offset without changing current semantics.
//! Tạo truy vấn registry và offset trang mà không đổi ngữ nghĩa hiện tại.

pub(in crate::commands::search) const SEARCH_PAGE_SIZE: u32 = 20;

pub(in crate::commands::search) struct SearchRequest {
    pub(in crate::commands::search) search_query: String,
    pub(in crate::commands::search) size: u32,
    pub(in crate::commands::search) from: u32,
    pub(in crate::commands::search) page: Option<u32>,
}

impl SearchRequest {
    pub(in crate::commands::search) fn new(query: String, exact: bool, page: Option<u32>) -> Self {
        let search_query = if exact {
            format!("exact:{}", query)
        } else {
            query.clone()
        };
        let size = SEARCH_PAGE_SIZE;
        let from = page.unwrap_or(1).saturating_sub(1) * size;

        Self {
            search_query,
            size,
            from,
            page,
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
