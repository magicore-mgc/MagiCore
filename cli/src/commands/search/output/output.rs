//! Serialize and emit package-search results in the existing CLI formats.
//! Serialize và hiển thị kết quả search theo format CLI hiện tại.

use mgc_ui::info;

use super::super::{
    error::SearchError,
    query::SEARCH_PAGE_SIZE,
    types::{SearchOutput, SearchResult},
};

pub(in crate::commands::search) fn report_no_results() {
    info("No results found");
}

pub(in crate::commands::search) fn present_results(
    total: u64,
    object_count: usize,
    results: Vec<SearchResult>,
    json: bool,
    page: Option<u32>,
    core_filter: Option<&str>,
) -> Result<(), SearchError> {
    if json {
        println!("{}", render_json(total, results)?);
        return Ok(());
    }

    emit_text_lines(
        total,
        object_count,
        &results,
        page,
        core_filter,
        |line| match line {
            SearchTextLine::Information(message) => info(&message),
            SearchTextLine::Blank => mgc_ui::blank_line(),
        },
    );

    Ok(())
}

fn render_json(total: u64, results: Vec<SearchResult>) -> Result<String, SearchError> {
    let output = SearchOutput { total, results };
    Ok(serde_json::to_string_pretty(&output)?)
}

fn emit_text_lines(
    total: u64,
    object_count: usize,
    results: &[SearchResult],
    page: Option<u32>,
    core_filter: Option<&str>,
    mut emit: impl FnMut(SearchTextLine),
) {
    let filter_suffix = core_filter
        .map(|filter| format!(" [filter: --core {}]", filter))
        .unwrap_or_default();
    emit(SearchTextLine::Information(format!(
        "Found {} result(s) (total: {}){}:",
        results.len(),
        total,
        filter_suffix
    )));
    emit(SearchTextLine::Blank);

    for result in results {
        emit(SearchTextLine::Information(format!(
            "  {} {}@{}",
            result.core_label, result.name, result.version
        )));
        if !result.description.is_empty() {
            emit(SearchTextLine::Information(format!(
                "    {}",
                result.description
            )));
        }
    }

    if total > object_count as u64 {
        let current = page.unwrap_or(1);
        emit(SearchTextLine::Information(format!(
            "Page {} of ~{}",
            current,
            total.div_ceil(SEARCH_PAGE_SIZE as u64)
        )));
    }
}

enum SearchTextLine {
    Information(String),
    Blank,
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
