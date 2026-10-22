//! Parse direct Go module require pins for the OSV fallback.
//! (Parse pin require Go trực tiếp cho OSV fallback.)

use crate::scanners::osv::OsvPin;

pub fn read_go_mod_requires(raw: &str) -> (Vec<OsvPin>, Vec<String>) {
    let mut pins = Vec::new();
    let mut skipped = Vec::new();
    let mut in_require_block = false;
    for line in raw.lines() {
        // Strip `//` comments — versions never contain them.
        // Bỏ comment `//` — version không bao giờ chứa chúng.
        let code = line.split("//").next().unwrap_or("").trim();
        if code.is_empty() {
            continue;
        }
        if code.starts_with("require (") || code == "require(" {
            in_require_block = true;
            continue;
        }
        if in_require_block && code == ")" {
            in_require_block = false;
            continue;
        }
        let directive = if in_require_block {
            Some(code)
        } else if let Some(rest) = code.strip_prefix("require ") {
            let rest = rest.trim();
            if rest.starts_with('(') || rest.is_empty() {
                in_require_block = true;
                None
            } else {
                Some(rest)
            }
        } else {
            if code.starts_with("replace ")
                || code.starts_with("exclude ")
                || code.starts_with("retract")
                || code.starts_with("toolchain ")
                || code.starts_with("go ")
            {
                skipped.push(format!("go.mod directive not advisory-mapped: {code}"));
            }
            None
        };
        let Some(req) = directive else { continue };
        let mut parts = req.split_whitespace();
        match (parts.next(), parts.next()) {
            (Some(module), Some(version))
                if !module.is_empty()
                    && version.starts_with('v')
                    && version.chars().nth(1).is_some_and(|c| c.is_ascii_digit()) =>
            {
                pins.push(OsvPin {
                    name: module.to_string(),
                    version: version.to_string(),
                    ecosystem: "Go",
                });
            }
            _ => skipped.push(format!("go.mod require line without a version pin: {req}")),
        }
    }
    (pins, skipped)
}
