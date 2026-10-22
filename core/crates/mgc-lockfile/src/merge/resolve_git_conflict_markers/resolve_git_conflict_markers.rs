// Git conflict marker parsing and merge delegation.
// Parse Git conflict marker và giao việc merge.
use crate::merge::merge3;
use crate::{Lockfile, parser};

/// Parses and resolves Git conflict markers when both sides are canonical lockfiles.
/// Parse và giải quyết Git conflict marker khi hai phía là lockfile canonical.
pub fn resolve_git_conflict_markers(content: &str) -> Option<Lockfile> {
    if !content.contains("<<<<<<<") || !content.contains("=======") || !content.contains(">>>>>>>")
    {
        return None;
    }

    let mut ours_lines = Vec::new();
    let mut theirs_lines = Vec::new();
    let mut in_ours = false;
    let mut in_theirs = false;

    for line in content.lines() {
        if line.starts_with("<<<<<<<") {
            in_ours = true;
            in_theirs = false;
        } else if line.starts_with("=======") {
            in_ours = false;
            in_theirs = true;
        } else if line.starts_with(">>>>>>>") {
            in_ours = false;
            in_theirs = false;
        } else if in_ours {
            ours_lines.push(line);
        } else if in_theirs {
            theirs_lines.push(line);
        } else {
            // Common lines outside markers belong to both parsed sides.
            // Dòng chung ngoài marker được đưa vào cả hai phía để parse.
            ours_lines.push(line);
            theirs_lines.push(line);
        }
    }

    let ours_lock = parse_conflict_side(&ours_lines)?;
    let theirs_lock = parse_conflict_side(&theirs_lines)?;

    // An empty base synthesizes the two parsed branches through the same merge rules.
    // Base rỗng tổng hợp hai nhánh đã parse theo cùng quy tắc merge hiện hành.
    let base_lock = Lockfile::new();
    merge3(&base_lock, &ours_lock, &theirs_lock).ok()
}

/// Parse one conflict side, treating an omitted package table as an empty side.
/// Parse một phía conflict, coi bảng package bị lược bỏ là phía rỗng.
fn parse_conflict_side(lines: &[&str]) -> Option<Lockfile> {
    let has_package_table = lines.iter().any(|line| {
        let trimmed = line.trim();
        trimmed == "[[package]]"
            || trimmed
                .split_once('=')
                .is_some_and(|(key, _)| key.trim() == "package")
    });

    if has_package_table {
        return parser::parse_lockfile(&lines.join("\n")).ok();
    }

    // Put the empty top-level array before [metadata], because TOML table
    // context continues until another table header.
    let mut normalized = Vec::with_capacity(lines.len() + 1);
    normalized.push("package = []");
    normalized.extend_from_slice(lines);
    parser::parse_lockfile(&normalized.join("\n")).ok()
}
