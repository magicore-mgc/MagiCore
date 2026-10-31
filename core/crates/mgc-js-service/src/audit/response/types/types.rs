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
    /// Real CVE identifiers carried by the advisory, e.g.
    /// `["CVE-2021-23337"]`. Optional on the bulk endpoint (older mirrors
    /// omit it), so it defaults to empty and falls back to the GHSA id in
    /// `url` and finally to the numeric `id`.
    /// CVE thật đi kèm advisory, vd `["CVE-2021-23337"]`. Tùy chọn trên
    /// bulk endpoint (mirror cũ bỏ trống), mặc định rỗng rồi fallback về
    /// GHSA trong `url` và cuối cùng là `id` số.
    #[serde(default)]
    pub(in crate::audit::response) cves: Vec<String>,
}

impl AdvisoryRecord {
    /// Best identifier to surface for this advisory.
    ///
    /// Precedence follows triage value: a real `CVE-<year>-<number>` first,
    /// then a GHSA id from the `cves` array, then a GHSA id in `url`, and
    /// only then the registry numeric id — labelled `Advisory` by the
    /// renderer, never `CVE`. Malformed CVE entries are skipped instead of
    /// being promoted to a CVE label.
    /// (Định danh tốt nhất để hiển thị. Ưu tiên giá trị triage: CVE thật
    /// trước, rồi GHSA trong mảng `cves`, rồi GHSA trong `url`, cuối cùng
    /// là id số registry — renderer gắn nhãn `Advisory`, không bao giờ
    /// `CVE`. Entry CVE sai dạng bị bỏ qua, không được nâng thành nhãn CVE.)
    pub(in crate::audit::response) fn primary_identifier(&self) -> String {
        if let Some(cve) = self.cves.iter().find(|value| is_cve_identifier(value)) {
            return cve.trim().to_string();
        }
        if let Some(ghsa) = self.cves.iter().find(|value| is_ghsa_identifier(value)) {
            return ghsa.trim().to_string();
        }
        if let Some(ghsa) = ghsa_from_url(&self.url) {
            return ghsa;
        }
        self.id.to_string()
    }
}

/// True only for a well-formed `CVE-<4 digits>-<number>` identifier.
/// (Đúng chỉ với id `CVE-<4 chữ số>-<số>` hợp lệ.)
pub(crate) fn is_cve_identifier(id: &str) -> bool {
    let id = id.trim();
    let Some(rest) = id.strip_prefix("CVE-").or_else(|| id.strip_prefix("cve-")) else {
        return false;
    };
    let Some((year, number)) = rest.split_once('-') else {
        return false;
    };
    year.len() == 4
        && year.bytes().all(|byte| byte.is_ascii_digit())
        && !number.is_empty()
        && number.bytes().all(|byte| byte.is_ascii_digit())
}

/// True for a `GHSA-xxxx-xxxx-xxxx` style identifier.
/// (Đúng với định danh dạng `GHSA-xxxx-xxxx-xxxx`.)
fn is_ghsa_identifier(id: &str) -> bool {
    let id = id.trim();
    id.len() > "GHSA-".len()
        && id
            .get(..5)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("GHSA-"))
}

/// Extract a `GHSA-...` id from an advisory URL path segment.
/// (Trích id `GHSA-...` từ một segment path của URL advisory.)
fn ghsa_from_url(url: &str) -> Option<String> {
    url.split(['/', '?', '#'])
        .map(|segment| segment.trim())
        .find(|segment| is_ghsa_identifier(segment))
        .map(str::to_string)
}
