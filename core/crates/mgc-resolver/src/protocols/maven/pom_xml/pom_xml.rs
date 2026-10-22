use mgc_types::{MgError, MgResult};

/// Validate Maven XML with an XML tokenizer before the small, bounded POM
/// field scanners run. Comments are removed so examples/documentation cannot
/// become fake dependencies; CDATA is converted to escaped text so markup in
/// CDATA is never interpreted as structure. DTDs are rejected because
/// external/general entity expansion is outside the supported POM contract.
pub(in crate::protocols::maven) fn normalize_pom_xml(pom: &str) -> MgResult<String> {
    use quick_xml::events::{BytesEnd, BytesText, Event};
    use quick_xml::{Reader, Writer};

    let mut reader = Reader::from_str(pom);
    reader.config_mut().trim_text(false);
    let mut writer = Writer::new(Vec::with_capacity(pom.len()));
    loop {
        let event = reader
            .read_event()
            .map_err(|error| MgError::Other(format!("invalid Maven POM XML: {error}")))?;
        match event {
            Event::Eof => break,
            Event::Comment(_) => {}
            Event::DocType(_) => {
                return Err(MgError::Other(
                    "Maven POM document types are unsupported; remove the DOCTYPE and retry".into(),
                ));
            }
            Event::CData(text) => {
                let decoded = text.xml10_content();
                writer
                    .write_event(Event::Text(BytesText::new(decoded.as_ref())))
                    .map_err(|error| MgError::Other(format!("normalize Maven POM XML: {error}")))?;
            }
            Event::Start(mut start) => {
                let local_name = start.local_name().into_inner().to_owned();
                start.set_name(&local_name);
                writer
                    .write_event(Event::Start(start.into_owned()))
                    .map_err(|error| MgError::Other(format!("normalize Maven POM XML: {error}")))?;
            }
            Event::Empty(mut empty) => {
                let local_name = empty.local_name().into_inner().to_owned();
                empty.set_name(&local_name);
                writer
                    .write_event(Event::Empty(empty.into_owned()))
                    .map_err(|error| MgError::Other(format!("normalize Maven POM XML: {error}")))?;
            }
            Event::End(end) => {
                let local_name = end.local_name().into_inner();
                writer
                    .write_event(Event::End(BytesEnd::new(local_name)))
                    .map_err(|error| MgError::Other(format!("normalize Maven POM XML: {error}")))?;
            }
            other => writer
                .write_event(other.into_owned())
                .map_err(|error| MgError::Other(format!("normalize Maven POM XML: {error}")))?,
        }
    }
    String::from_utf8(writer.into_inner())
        .map_err(|error| MgError::Other(format!("Maven POM is not UTF-8 XML: {error}")))
}

/// Return the inner text of every `<tag>…</tag>` block (same-tag nesting is
/// counted; self-closing and attribute-carrying open tags are handled).
/// Trả nội dung bên trong của mọi khối `<tag>…</tag>` (đếm lồng cùng tên;
/// xử lý tag tự đóng và tag mở có attribute).
pub fn element_blocks<'a>(xml: &'a str, tag: &str) -> Vec<&'a str> {
    let open_prefix = format!("<{tag}");
    let close = format!("</{tag}>");
    let mut out = Vec::new();
    let mut rest = xml;
    while let Some(start) = rest.find(&open_prefix) {
        let after = &rest[start + open_prefix.len()..];
        // Boundary check: `<version>` must not match `<versions>`.
        // (Kiểm tra biên: `<version>` không được khớp `<versions>`.)
        let boundary_ok = after.starts_with('>')
            || after.starts_with('/')
            || after.starts_with(|c: char| c.is_whitespace());
        if !boundary_ok {
            rest = &rest[start + open_prefix.len()..];
            continue;
        }
        if after.starts_with('/') {
            // Self-closing `<tag/>` — no content.
            // (Tự đóng `<tag/>` — không có nội dung.)
            let Some(gt) = rest[start..].find('>') else {
                break;
            };
            rest = &rest[start + gt + 1..];
            continue;
        }
        let Some(gt_rel) = rest[start..].find('>') else {
            break;
        };
        let gt = start + gt_rel;
        // Self-closing with attributes (`<dependency id="x" />`) — empty
        // content, keep scanning (a missing close tag must not abort the
        // whole scan).
        // (Tự đóng có attribute (`<dependency id="x" />`) — nội dung rỗng,
        // quét tiếp (thiếu close tag không được làm hỏng cả vòng quét).)
        if rest[start..=gt].ends_with("/>") {
            out.push("");
            rest = &rest[gt + 1..];
            continue;
        }
        let content_start = gt + 1;
        let mut depth = 1usize;
        let mut cursor = content_start;
        let mut end = None;
        while let Some(idx) = rest[cursor..].find('<') {
            let abs = cursor + idx;
            if rest[abs..].starts_with(&close) {
                depth -= 1;
                if depth == 0 {
                    end = Some(abs);
                    break;
                }
                cursor = abs + close.len();
                continue;
            }
            if rest[abs..].starts_with(&open_prefix) {
                let after2 = &rest[abs + open_prefix.len()..];
                if after2.starts_with('>') || after2.starts_with(|c: char| c.is_whitespace()) {
                    depth += 1;
                }
            }
            cursor = abs + 1;
        }
        match end {
            Some(e) => {
                out.push(&rest[content_start..e]);
                rest = &rest[e + close.len()..];
            }
            None => break,
        }
    }
    out
}

/// Remove every `<tag>…</tag>` block (used to strip
/// `dependencyManagement` before dependency collection).
/// Xóa mọi khối `<tag>…</tag>` (dùng để cắt `dependencyManagement` trước
/// khi gom dependency).
pub(in crate::protocols::maven) fn remove_element_blocks(xml: &str, tag: &str) -> String {
    let blocks = element_blocks(xml, tag);
    let mut out = xml.to_string();
    for block in blocks {
        // element_blocks returns non-overlapping, in-order slices — replace
        // each `<tag>…</tag>` occurrence once.
        // (element_blocks trả các slice không chồng lấn, theo thứ tự — thay
        // mỗi lần xuất hiện `<tag>…</tag>` đúng một lần.)
        let full = format!("<{tag}>{block}</{tag}>");
        out = out.replacen(&full, "", 1);
    }
    out
}
