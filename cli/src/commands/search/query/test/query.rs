//! Pin search query and pagination behavior.
//! Cố định hành vi truy vấn và phân trang tìm kiếm.

use super::super::SearchRequest;

#[test]
fn preserves_exact_query_marker_and_plain_query() {
    let exact = SearchRequest::new("react".to_string(), true, None);
    let plain = SearchRequest::new("react".to_string(), false, None);

    assert_eq!(exact.search_query, "exact:react");
    assert_eq!(plain.search_query, "react");
}

#[test]
fn preserves_existing_page_offset_calculation() {
    let first_page = SearchRequest::new("pkg".to_string(), false, None);
    let explicit_first_page = SearchRequest::new("pkg".to_string(), false, Some(1));
    let second_page = SearchRequest::new("pkg".to_string(), false, Some(2));
    let zero_page = SearchRequest::new("pkg".to_string(), false, Some(0));

    assert_eq!(first_page.size, 20);
    assert_eq!(first_page.from, 0);
    assert_eq!(explicit_first_page.from, 0);
    assert_eq!(second_page.from, 20);
    assert_eq!(zero_page.from, 0);
}
