//! Shared tag attribute and raw-block scanning used by NuGet manifests.
//! Quét attribute và khối tag dùng chung cho manifest NuGet.

/// Read attribute `attr="…"` from the FIRST opening tag in `xml` (up to the
/// first `>` — nested tags never contaminate the lookup).
/// Đọc attribute `attr="…"` từ tag mở ĐẦU TIÊN trong `xml` (đến `>` đầu
/// tiên — tag lồng không bao giờ làm bẩn kết quả).
pub(in crate::protocols::nuget) fn tag_attr(xml: &str, attr: &str) -> Option<String> {
    let lt = xml.find('<')?;
    let gt_rel = xml[lt..].find('>')?;
    let tag = &xml[lt..lt + gt_rel];
    let needle = format!("{attr}=");
    let pos = tag.find(&needle)?;
    let rest = tag[pos + needle.len()..].trim_start();
    let quote = rest.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    rest[1..].split(quote).next().map(str::to_string)
}

/// Extract each `<tag …>…</tag>` / `<tag …/>` raw block.
/// Lấy từng khối thô `<tag …>…</tag>` / `<tag …/>`.
pub(in crate::protocols::nuget) fn tag_blocks<'a>(xml: &'a str, tag: &str) -> Vec<&'a str> {
    let open_prefix = format!("<{tag}");
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(start) = rest.find(&open_prefix) {
        let after = &rest[start + open_prefix.len()..];
        if !(after.starts_with('>')
            || after.starts_with('/')
            || after.starts_with(|c: char| c.is_whitespace()))
        {
            rest = &rest[start + open_prefix.len()..];
            continue;
        }
        let Some(gt_rel) = rest[start..].find('>') else {
            break;
        };
        let gt = start + gt_rel;
        if rest[start..gt + 1].ends_with("/>") {
            out.push(&rest[start..gt + 1]);
            rest = &rest[gt + 1..];
            continue;
        }
        let close = format!("</{tag}>");
        match rest[gt..].find(&close) {
            Some(end_rel) => {
                let end = gt + end_rel + close.len();
                out.push(&rest[start..end]);
                rest = &rest[end..];
            }
            None => break,
        }
    }
    out
}
