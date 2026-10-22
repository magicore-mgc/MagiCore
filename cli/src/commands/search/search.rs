//! Coordinate package-search input, registry access, mapping and output.
//! Điều phối input tìm package, truy vấn registry, ánh xạ và hiển thị kết quả.

use anyhow::Result;

use super::{
    output::{present_results, report_no_results},
    query::SearchRequest,
    registry_request::fetch_registry_search,
    results::map_and_filter_results,
};

/// Run package search using the global core-filter environment value.
/// Chạy tìm package với giá trị lọc core từ biến môi trường toàn cục.
pub async fn run(query: String, json: bool, exact: bool, page: Option<u32>) -> Result<()> {
    // Read the global CLI filter while preserving the existing public signature.
    // Đọc bộ lọc CLI toàn cục và giữ nguyên chữ ký công khai hiện có.
    let core_filter = std::env::var("MGC_CORE_FILTER").ok();
    run_with_core(query, json, exact, page, core_filter.as_deref()).await
}

/// Run package search with an explicitly supplied core filter.
/// Chạy tìm package với bộ lọc core được truyền tường minh.
pub async fn run_with_core(
    query: String,
    json: bool,
    exact: bool,
    page: Option<u32>,
    core_filter: Option<&str>,
) -> Result<()> {
    let request = SearchRequest::new(query, exact, page);
    let response = fetch_registry_search(&request).await?;

    if response.objects.is_empty() {
        report_no_results();
        return Ok(());
    }

    let results = map_and_filter_results(&response.objects, core_filter);
    present_results(
        response.total,
        response.objects.len(),
        results,
        json,
        request.page,
        core_filter,
    )?;

    Ok(())
}
