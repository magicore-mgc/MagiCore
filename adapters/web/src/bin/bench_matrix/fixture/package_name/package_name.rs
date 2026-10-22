// Registry package path formatter
// Chuẩn hóa tên package thành đường dẫn tarball fixture.
pub(in crate::bench_matrix) fn sanitize_pkg_name(name: &str) -> String {
    name.replace('/', "__").replace('@', "")
}
