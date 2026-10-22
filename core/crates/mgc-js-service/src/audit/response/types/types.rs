// Required npm advisory schema fields. / Các field schema advisory npm bắt buộc.
/// Typed advisory record — the REAL npm Bulk Advisory API schema
/// (verified against the npm bulk endpoint contract; the same shape
/// pnpm audit consumes). The registry does NOT return `findings` —
/// clients build findings by matching `vulnerable_versions` against the
/// local dependency tree (Tech Lead P0-1 2026-09-09).
/// Bản ghi advisory typed — schema THẬT của npm Bulk Advisory API (cùng
/// shape mà pnpm audit dùng). Registry KHÔNG trả `findings` — client tự
/// đối chiếu `vulnerable_versions` với dependency tree local.
///
/// Required: id, url, title, severity, vulnerable_versions. A response
/// missing any of these is malformed and must fail closed.
/// Bắt buộc: id, url, title, severity, vulnerable_versions. Response thiếu
/// field nào là malformed và phải fail-closed.
#[derive(serde::Deserialize)]
pub(in crate::audit) struct AdvisoryRecord {
    /// Numeric advisory id from the registry.
    /// Id advisory số từ registry.
    pub(in crate::audit::response) id: serde_json::Number,
    pub(in crate::audit::response) url: String,
    pub(in crate::audit::response) title: String,
    pub(in crate::audit::response) severity: String,
    /// Semver range of affected versions, e.g. "<4.17.21".
    /// Range semver của các version dính, vd "<4.17.21".
    pub(in crate::audit::response) vulnerable_versions: String,
}
