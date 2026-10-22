# Resolve model owner

Owns the recursive tree HTTP request and orchestration: validate first, GET, read body, check status, parse JSON, then delegate entry validation and sorting. It preserves the old endpoint and error ordering. Black-box request behavior is covered by core/crates/mgc-resolver/tests/protocols_hfhub.rs.

Phụ trách request HTTP tree đệ quy và điều phối: kiểm tra input trước, GET, đọc body, kiểm tra status, parse JSON rồi giao kiểm tra/sắp xếp entry. Giữ endpoint và thứ tự lỗi cũ. Hành vi request black-box được bao phủ tại core/crates/mgc-resolver/tests/protocols_hfhub.rs.
