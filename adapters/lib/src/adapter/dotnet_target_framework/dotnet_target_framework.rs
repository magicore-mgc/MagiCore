use crate::language::find_csproj;
use mgc_types::MgResult;
use std::path::Path;

/// Read the consumer target framework from the unique root `*.csproj`.
/// Missing/ambiguous projects return `None`; filesystem/parse-read failures
/// propagate so callers cannot silently resolve without the target context.
/// (Đọc framework từ csproj root duy nhất; lỗi đọc không bị nuốt.)
pub(crate) fn read_dotnet_target_framework(root: &Path) -> MgResult<Option<String>> {
    let Some(csproj) = find_csproj(root) else {
        return Ok(None);
    };
    read_dotnet_target_framework_file(&csproj)
}

/// Read and parse a selected csproj through the shared no-follow handle.
/// Kept separate so symlink races can be regression-tested directly.
/// (Đọc csproj qua handle no-follow; tách riêng để test symlink-race.)
pub(crate) fn read_dotnet_target_framework_file(path: &Path) -> MgResult<Option<String>> {
    let content = mgc_adapter_base::project_file::read_regular_text(path, ".csproj")?;
    let tag = |name: &str| -> Option<String> {
        let open = format!("<{name}>");
        let close = format!("</{name}>");
        let start = content.find(&open)? + open.len();
        let end = content[start..].find(&close)?;
        Some(content[start..start + end].trim().to_string())
    };
    if let Some(single) = tag("TargetFramework").filter(|v| !v.is_empty()) {
        return Ok(Some(single));
    }
    let Some(multi) = tag("TargetFrameworks").filter(|v| !v.is_empty()) else {
        return Ok(None);
    };
    let mut frameworks = multi.split(';').map(str::trim).filter(|v| !v.is_empty());
    let Some(first) = frameworks.next().map(str::to_string) else {
        return Ok(None);
    };
    if frameworks.next().is_some() {
        eprintln!(
            "WARNING: multi-target project ({multi}) — native resolve selects dependency groups for '{first}' only"
        );
    }
    Ok(Some(first))
}
