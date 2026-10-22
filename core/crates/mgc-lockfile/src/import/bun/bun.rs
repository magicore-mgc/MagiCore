//! Parse Bun text lockfile JSONC — parse JSONC lockfile Bun dạng text.

use anyhow::{Result, bail};
use mgc_types::strip_jsonc;

use crate::schema::Package;

use super::super::{SkippedRecord, split_name_version};

// ---------------------------------------------------------------------------
// bun — bun.lock JSON (text; .lockb binary ngoài phạm vi)
// P0 finding #8 (2026-09-12): BEST-EFFORT, not lossless — bun stores
// RANGES in the dep map while mgc.lock pins; the edge keeps the dep NAME
// and the range is recorded as a loss (the source file remains the
// range source of truth). Every refused entry is EXPLICITLY appended to
// `skipped` — silent `continue` is gone. Entry shapes:
// ["name@version", "url?", {deps}, "integrity?"].
// (bun.lock text JSON — .lockb binary out of scope. Best-effort: bun
// keeps RANGES, mgc.lock pins — dep NAME survives as the edge, the
// range is a recorded loss; every refused entry lands in `skipped`.)
// ---------------------------------------------------------------------------

pub(in crate::import) fn parse_bun(
    content: &str,
    skipped: &mut Vec<SkippedRecord>,
) -> Result<Vec<Package>> {
    // P0-3 (2026-09-10): bun's writer emits JSONC (line comments +
    // trailing commas) — the same shape mgc-audit's lockfile reader
    // already handles. Import must accept what real `bun install` writes,
    // else `mgc import bun` fails on the very fixtures the migration
    // targets. Strip JSONC BEFORE the strict JSON parse — the shared
    // mgc-types::jsonc implementation (P1 dedup, 2026-09-11: one
    // implementation, two consumers, same contract).
    // Writer của bun ghi JSONC (comment dòng + dấu phẩy cuối) — import
    // phải chấp nhận đúng cái bun install ghi ra, nếu không `mgc import
    // bun` fail trên chính fixture migration nhắm tới. Cắt JSONC trước
    // khi parse JSON strict — dùng bản strip_jsonc chung trong
    // mgc-types::jsonc (dedup P1: một bản implement, hai nơi dùng).
    let cleaned = strip_jsonc(content);
    let json: serde_json::Value = serde_json::from_str(&cleaned)
        .map_err(|e| anyhow::anyhow!("bun.lock is not valid JSONC: {e}"))?;

    if let Some(version) = json.get("lockfileVersion") {
        let version = version
            .as_u64()
            .ok_or_else(|| anyhow::anyhow!("bun.lock has a non-integer 'lockfileVersion'"))?;
        if version > 2 {
            bail!("bun lockfileVersion {version} is not supported yet — refusing to guess");
        }
    }

    let entries = json
        .get("packages")
        .and_then(|v| v.as_object())
        .ok_or_else(|| anyhow::anyhow!("bun.lock has no 'packages' map"))?;

    let mut out = Vec::new();
    for (key, entry) in entries {
        // A standard entry is ["name@version", "tarball-url"?, {deps}?, "sha512-..."?].
        // Entry chuẩn có dạng ["name@version", "tarball-url"?, {deps}?, "sha512-..."?].
        let Some(items) = entry.as_array() else {
            skipped.push(SkippedRecord {
                key: key.clone(),
                reason: "entry is not an array".to_string(),
            });
            continue;
        };
        let Some(spec) = items.first().and_then(|v| v.as_str()) else {
            skipped.push(SkippedRecord {
                key: key.clone(),
                reason: "first entry element is not a name@version string".to_string(),
            });
            continue;
        };
        let Some((name, version)) = split_name_version(spec) else {
            // Workspace/registry ids that are not npm pins: RECORDED
            // (P0-5) — e.g. "workspace:root" survives in the report.
            // Id workspace/registry không phải pin npm: ĐƯỢC GHI
            // (P0-5) — vd "workspace:root" sống sót trong report.
            skipped.push(SkippedRecord {
                key: key.clone(),
                reason: format!("entry spec '{spec}' is not a name@version pin"),
            });
            continue;
        };
        let mut resolved = String::new();
        let mut integrity = String::new();
        let mut dependencies: Vec<String> = Vec::new();
        for item in items.iter().skip(1) {
            if let Some(text) = item.as_str() {
                if text.starts_with("http://") || text.starts_with("https://") {
                    resolved = text.to_string();
                } else if text.contains("sha512-") || text.contains("sha256-") {
                    integrity = text.to_string();
                }
            }
        }
        // Bun's third package-array element is INFO, whose top-level keys
        // also include peerDependencies, optionalPeers, os, cpu and bin.
        // Only the nested production `dependencies` map represents the
        // dependency edges imported by the pnpm/npm paths. Bun records
        // ranges here rather than resolved IDs, so preserve names only.
        // Phần tử INFO có nhiều khóa metadata; chỉ map dependencies lồng
        // bên trong mới là cạnh package thường. Bun chỉ ghi range nên giữ
        // tên dependency, không tự suy diễn phiên bản.
        if let Some(deps) = items
            .get(2)
            .and_then(|info| info.get("dependencies"))
            .and_then(serde_json::Value::as_object)
        {
            dependencies.extend(deps.keys().cloned());
        }
        out.push(Package {
            name,
            version,
            resolved,
            integrity,
            dependencies,
            ..Default::default()
        });
    }
    Ok(out)
}

// The bun JSONC strip (quote-aware comments + trailing commas) was
// consolidated into mgc-types::jsonc::strip_jsonc (P1 dedup 2026-09-11)
// — this file now imports the single shared implementation above.
// Phần strip JSONC của bun (comment nhận-biết-quote + dấu phẩy cuối)
// đã gộp vào mgc-types::jsonc::strip_jsonc (dedup P1) — file này giờ
// dùng bản chung duy nhất ở trên.
