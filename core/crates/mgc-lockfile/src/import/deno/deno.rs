//! Parse Deno lockfile JSON — parse JSON lockfile Deno.

use anyhow::Result;

use crate::schema::Package;

use super::super::{SkippedRecord, split_name_version};

// ---------------------------------------------------------------------------
// deno — deno.lock v5 (JSON): npm map "name@version" → integrity; jsr map
// deno — parser lấy pin npm và JSR từ schema JSON v5.
// giữ nguyên tiền tố "jsr:" để audit báo skipped trung thực (P0-3 2026-09-10).
// P0 finding #6 (2026-09-12): root pins từ workspace.dependencies là
// GRAPH ROOT — ghi vào lockfile.root_dependencies, KHÔNG self-edge vào
// package. P0 finding #8: import là BEST-EFFORT kèm loss report — mọi
// record bị từ chối liệt kê tường minh trong `skipped`; JSR pin giữ
// tiền tố "jsr:" và luôn có lý do no-npm-advisory đi kèm.
// (deno.lock v5: npm map name@version → integrity; jsr keeps the jsr:
// prefix so audit reports honestly. Root pins are the ROOT GRAPH —
// lockfile.root_dependencies, never a package self-edge. Best-effort
// migration: every refused record is explicitly listed in `skipped`.)
// ---------------------------------------------------------------------------

pub(in crate::import) fn parse_deno(
    content: &str,
    skipped: &mut Vec<SkippedRecord>,
    root_deps: &mut Vec<String>,
) -> Result<Vec<Package>> {
    let json: serde_json::Value = serde_json::from_str(content)
        .map_err(|e| anyhow::anyhow!("deno.lock is not valid JSON: {e}"))?;

    // Shape-first requires the "npm" map in deno.lock v5; "jsr" is optional.
    // Shape-first yêu cầu map "npm" trong deno.lock v5; map "jsr" là tùy chọn.
    let npm = json
        .get("npm")
        .and_then(|v| v.as_object())
        .ok_or_else(|| anyhow::anyhow!("deno.lock has no 'npm' map (v5 schema expected)"))?;

    // Root dependency pins (v5 "workspace"."dependencies"): raw pin list
    // used to rebuild direct-dependency edges for the root set.
    // Ghim dependency gốc (v5 "workspace"."dependencies"): danh sách pin
    // thô dùng để dựng lại cạnh dependency-trực-tiếp cho tập root.
    let root_pins: Vec<String> = json
        .get("workspace")
        .and_then(|w| w.get("dependencies"))
        .and_then(|d| d.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let mut out = Vec::new();
    for (pin, entry) in npm {
        let Some((name, version)) = split_name_version(pin) else {
            // P0-5: malformed pins are RECORDED, never silently skipped.
            // P0-5: pin malformed được GHI LẠI, không bao giờ bỏ âm thầm.
            skipped.push(SkippedRecord {
                key: pin.clone(),
                reason: "npm pin is not name@version".to_string(),
            });
            continue;
        };
        // deno.lock v5 npm values come in TWO shapes: an object with
        // integrity ({"integrity": "sha512-..."}) or a bare version
        // string ("4.17.20") — both are valid real-world writer
        // output; accept both (string = no integrity recorded).
        // Giá trị npm của deno.lock v5 có HAI dạng: object chứa
        // integrity hoặc chuỗi version trơn — cả hai đều là output
        // writer thật; chấp nhận cả hai (string = không ghi integrity).
        let integrity = match entry {
            serde_json::Value::Object(obj) => obj
                .get("integrity")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            serde_json::Value::String(s) => {
                // Bare version string — cross-check against the pin's
                // version half; mismatch is a corrupt record.
                // Chuỗi version trơn — đối chiếu với nửa version của
                // pin; lệch nhau là record hỏng.
                if s != &version {
                    skipped.push(SkippedRecord {
                        key: pin.clone(),
                        reason: format!("npm version mismatch: pin says {version}, entry says {s}"),
                    });
                    continue;
                }
                String::new()
            }
            other => {
                skipped.push(SkippedRecord {
                    key: pin.clone(),
                    reason: format!("npm entry is neither object nor version string: {other}"),
                });
                continue;
            }
        };

        // Root direct dependencies: pins listed under
        // workspace.dependencies are the ROOT GRAPH (root → dep), NOT a
        // self-edge of the resolved package. They land in
        // lockfile.root_dependencies via the `root_deps` out-parameter
        // (P0 finding #6, 2026-09-12): the previous code pushed them
        // into the package's own `dependencies`, turning "workspace
        // root depends on lodash" into the lie "lodash depends on
        // npm:lodash@4.17.20". v5 pins carry runtime prefixes
        // ("npm:", "jsr:") — the pin is kept VERBATIM (prefix intact)
        // so no source information is lost.
        // Direct-dep gốc: pin trong workspace.dependencies là GRAPH
        // ROOT (root → dep), KHÔNG phải self-edge của package được
        // resolve. Chúng ghi vào lockfile.root_dependencies qua tham
        // số ra `root_deps` (P0 finding #6): code cũ đẩy vào
        // `dependencies` của chính package, biến "workspace root phụ
        // thuộc lodash" thành câu sai "lodash phụ thuộc
        // npm:lodash@4.17.20". Pin v5 mang tiền tố runtime — giữ
        // NGUYÊN pin, không mất thông tin nguồn.
        root_deps.extend(root_pins.iter().cloned());

        out.push(Package {
            name,
            version,
            resolved: String::new(),
            integrity,
            dependencies: vec![],
            ..Default::default()
        });
    }

    // JSR deps: không có advisory DB npm — import kèm tiền tố "jsr:" để
    // downstream (audit) báo skipped trung thực; P0-5: mỗi pin JSR đều
    // được liệt kê trong skipped kèm lý do (dữ liệu gốc không có npm
    // integrity/registry → không thể lossless-map sang npm semantics).
    // (JSR deps carry no npm advisory DB — the `jsr:` prefix survives so
    // downstream audit reports honestly; P0-5: every JSR pin is listed
    // in `skipped` with its reason — the source carries no npm
    // integrity/registry data, so a lossless npm mapping is impossible.)
    if let Some(jsr) = json.get("jsr").and_then(|v| v.as_object()) {
        for pin in jsr.keys() {
            let Some((name, version)) = split_name_version(pin) else {
                skipped.push(SkippedRecord {
                    key: format!("jsr:{pin}"),
                    reason: "jsr pin is not name@version".to_string(),
                });
                continue;
            };
            // Record the JSR skip BEFORE the values move into Package.
            // Ghi skip JSR TRƯỚC khi giá trị chuyển vào Package.
            skipped.push(SkippedRecord {
                key: format!("jsr:{name}@{version}"),
                reason: "jsr package has no npm registry/integrity mapping — audit reports it as skipped".to_string(),
            });
            out.push(Package {
                name: format!("jsr:{name}"),
                version,
                resolved: String::new(),
                integrity: String::new(),
                dependencies: vec![],
                ..Default::default()
            });
        }
    }

    Ok(out)
}
