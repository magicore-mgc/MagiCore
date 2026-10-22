//! .NET reference pruning.
//! Loại PackageReference .NET.

/// Drop `<PackageReference>` elements (single- or multi-line) whose
/// `Include` id is absent from `keep`. Anything else is preserved byte
/// for byte, including comments and whitespace.
pub(crate) fn prune_csproj_references(
    content: &str,
    keep: &std::collections::HashSet<&str>,
) -> String {
    fn kept(element: &str, keep: &std::collections::HashSet<&str>) -> bool {
        keep.iter().any(|id| {
            element.contains(&format!("Include=\"{id}\""))
                || element.contains(&format!("Include='{id}'"))
        })
    }
    let mut out = String::with_capacity(content.len());
    let mut rest = content;
    while let Some(start) = rest.find("<PackageReference") {
        out.push_str(&rest[..start]);
        let tail = &rest[start..];
        let tag_end = tail.find('>').map(|p| p + 1);
        let Some(tag_end) = tag_end else {
            out.push_str(tail);
            rest = "";
            break;
        };
        let opening = &tail[..tag_end];
        let element_end = if opening.ends_with("/>") {
            tag_end
        } else if let Some(fe) = tail
            .find("</PackageReference>")
            .map(|p| p + "</PackageReference>".len())
        {
            fe
        } else {
            out.push_str(tail);
            rest = "";
            break;
        };
        let element = &tail[..element_end];
        if kept(element, keep) {
            out.push_str(element);
            rest = &tail[element_end..];
        } else {
            // Dropped: swallow one following newline so no blank line
            // litters the ItemGroup.
            rest = tail[element_end..]
                .strip_prefix('\n')
                .unwrap_or(&tail[element_end..]);
        }
    }
    out.push_str(rest);
    out
}
