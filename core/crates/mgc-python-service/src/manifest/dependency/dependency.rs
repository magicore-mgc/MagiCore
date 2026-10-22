//! Simple PEP 508 parsing and normalized manifest conversion.
//! Phân tích PEP 508 đơn giản và chuyển đổi manifest chuẩn hóa.

use mgc_types::{DependencySpec, MgResult, PackageName, VersionRange};

use super::error::PythonDependencyError;

pub fn parse_python_dependency(spec: &str) -> MgResult<DependencySpec> {
    let trimmed = spec.trim();
    if trimmed.is_empty()
        || trimmed.chars().any(char::is_whitespace)
        || trimmed
            .chars()
            .any(|ch| matches!(ch, '[' | ']' | ';' | '@' | ','))
    {
        return Err(PythonDependencyError::UnsupportedSyntax.into());
    }
    let split_at = ["==", ">=", "<=", "~=", ">", "<"]
        .iter()
        .filter_map(|op| trimmed.find(op).map(|idx| (idx, *op)))
        .min_by_key(|(idx, _)| *idx);

    let Some((idx, op)) = split_at else {
        return DependencySpec::parse(trimmed);
    };

    let name = PackageName::new(trimmed[..idx].trim())?;
    let raw_range = trimmed[idx + op.len()..].trim();
    if raw_range.is_empty()
        || raw_range
            .chars()
            .any(|ch| !(ch.is_ascii_alphanumeric() || ".-*+^~=<>!|".contains(ch)))
    {
        return Err(PythonDependencyError::InvalidVersionRequirement(trimmed.to_string()).into());
    }
    let range = match op {
        "==" => VersionRange::parse(raw_range)?,
        "~=" => VersionRange::parse(&format!("~{raw_range}"))?,
        _ => VersionRange::parse(&format!("{op}{raw_range}"))?,
    };
    Ok(DependencySpec::new(name, range))
}

pub fn python_dependency_value(dep: &DependencySpec) -> toml::Value {
    // Star ranges remain bare PEP 508 names; exact pins stay exact, and
    // unsupported npm-style ranges are represented as lower bounds.
    // (Range `*` giữ tên trần, pin exact không bị nới rộng.)
    if dep.range.is_star() {
        return toml::Value::String(dep.name.as_str().to_string());
    }
    let raw = dep.range.as_str().trim();
    let bound = if let Some(version) = raw.strip_prefix("==").or_else(|| raw.strip_prefix('=')) {
        format!("=={version}")
    } else if let Some(version) = raw.strip_prefix("~=") {
        format!("~={version}")
    } else if raw.starts_with(">=") || raw.starts_with("<=") || raw.starts_with("!=") {
        raw.to_string()
    } else if let Some(version) = raw.strip_prefix('>') {
        format!(">{version}")
    } else if let Some(version) = raw.strip_prefix('<') {
        format!("<{version}")
    } else if let Some(version) = raw.strip_prefix('^').or_else(|| raw.strip_prefix('~')) {
        format!(">={version}")
    } else {
        format!("=={raw}")
    };
    toml::Value::String(format!("{}{bound}", dep.name.as_str()))
}
