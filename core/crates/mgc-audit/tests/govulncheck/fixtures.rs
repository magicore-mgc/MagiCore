// Go scanner stream fixtures copied byte-for-byte from the integration target.
// Fixture stream scanner Go được chuyển nguyên byte từ integration target.

/// A realistic vulnerable stream: SBOM + OSV + module-level finding.
/// Stream dính lỗi thực tế: SBOM + OSV + finding level module.
const VULNERABLE_STREAM: &str = r#"{"config":{"protocol_version":"v1.0.0","scanner_name":"govulncheck","scanner_version":"v1.8.0","db":"vuln.go.dev","go_version":"go1.26.4","scan_level":"symbol","scan_mode":"source"}}
{"progress":{"message":"Scanning your code and P packages across M dependent modules for known vulnerabilities..."}}
{"osv":{"id":"GO-2020-0013","summary":"Denial of service via crafted Content-Length header in golang.org/x/text","aliases":["CVE-2020-14040"],"severity":[],"references":[{"url":"https://go.dev/issue/39157"}]}}
{"osv":{"id":"GO-2021-0113","summary":"Wire is vulnerable to DoS in golang.org/x/crypto","details":"A maliciously crafted input can cause excessive memory allocation.","aliases":["CVE-2021-43565"],"severity":[],"database_specific":{"severity":"HIGH"},"references":[]}}
{"SBOM":{"go_version":"go1.26.4","modules":[{"path":"example.com/vuln","version":"v1.0.0"},{"path":"golang.org/x/text","version":"v0.3.2"}],"roots":["example.com/vuln"]}}
{"finding":{"osv":"GO-2020-0013","fixed_version":"v0.3.3","trace":[{"module":"golang.org/x/text","version":"v0.3.2"}]}}
{"finding":{"osv":"GO-2021-0113","fixed_version":"v0.0.0-20211202","trace":[{"module":"golang.org/x/crypto","version":"v0.0.0-20211202"}]}}
{"finding":{"osv":"GO-2021-0113","fixed_version":"v0.0.0-20211202","trace":[{"module":"golang.org/x/crypto","version":"v0.0.0-20211202","package":"golang.org/x/crypto/ssh"}]}}
"#;

const CLEAN_STREAM: &str = r#"{"config":{"protocol_version":"v1.0.0","scan_level":"symbol","scan_mode":"source"}}
{"progress":{"message":"Scanning..."}}
{"SBOM":{"go_version":"go1.26.4","modules":[{"path":"example.com/clean","version":"v1.0.0"}],"roots":["example.com/clean"]}}
"#;
