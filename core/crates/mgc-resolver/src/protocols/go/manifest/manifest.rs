//! go.mod model and line-oriented require/replace/exclude parser.
//! Model go.mod và parser require/replace/exclude theo dòng.

use mgc_types::{MgError, MgResult};

/// Parsed go.mod: requirements, replacements and exclusions.
/// go.mod đã parse: requirement, replace và exclude.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GoModFile {
    pub requires: Vec<GoRequire>,
    pub replaces: Vec<GoReplace>,
    pub excludes: Vec<(String, String)>,
}

/// One `require` line — `// indirect` captured as a flag, not a dep.
/// Một dòng `require` — `// indirect` ghi thành flag, không phải dep.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoRequire {
    pub path: String,
    pub version: String,
    pub indirect: bool,
}

/// One `replace` directive: `old [oldv] => new [newv]`.
/// Một directive `replace`: `old [oldv] => new [newv]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoReplace {
    pub old_path: String,
    pub old_version: Option<String>,
    pub new_path: String,
    pub new_version: Option<String>,
}

/// Parse go.mod text (line-based, comment-aware): single-line and
/// parenthesized `require` / `replace` / `exclude` blocks. `module`/`go`/
/// `toolchain`/`godebug` directives are ignored.
/// Parse nội dung go.mod (theo dòng, hiểu comment): khối `require` /
/// `replace` / `exclude` một dòng và trong ngoặc. Bỏ qua directive
/// `module`/`go`/`toolchain`/`godebug`.
pub fn parse_go_mod(content: &str) -> MgResult<GoModFile> {
    let mut file = GoModFile::default();
    #[derive(PartialEq, Eq, Clone, Copy)]
    enum Block {
        None,
        Require,
        Replace,
        Exclude,
    }
    let mut block = Block::None;

    for raw_line in content.lines() {
        let trimmed = raw_line.trim();
        let indirect = trimmed.contains("// indirect");
        // Strip trailing comments before tokenizing.
        // (Cắt comment cuối trước khi tách token.)
        let line = trimmed.split("//").next().unwrap_or("").trim();
        if line.is_empty() {
            if block != Block::None && trimmed.starts_with(')') {
                block = Block::None;
            }
            continue;
        }
        if block == Block::None {
            if let Some(rest) = line.strip_prefix("require") {
                let rest = rest.trim();
                if rest == "(" {
                    block = Block::Require;
                } else {
                    push_require(&mut file, rest, indirect)?;
                }
            } else if let Some(rest) = line.strip_prefix("replace") {
                let rest = rest.trim();
                if rest == "(" {
                    block = Block::Replace;
                } else {
                    push_replace(&mut file, rest)?;
                }
            } else if let Some(rest) = line.strip_prefix("exclude") {
                let rest = rest.trim();
                if rest == "(" {
                    block = Block::Exclude;
                } else {
                    push_exclude(&mut file, rest)?;
                }
            }
            continue;
        }
        // Inside a block: `)` closes, anything else is a directive body.
        // (Trong khối: `)` đóng, còn lại là thân directive.)
        if trimmed.starts_with(')') {
            block = Block::None;
            continue;
        }
        match block {
            Block::Require => push_require(&mut file, line, indirect)?,
            Block::Replace => push_replace(&mut file, line)?,
            Block::Exclude => push_exclude(&mut file, line)?,
            Block::None => {}
        }
    }
    Ok(file)
}

fn push_require(file: &mut GoModFile, body: &str, indirect: bool) -> MgResult<()> {
    let mut parts = body.split_whitespace();
    let Some(path) = parts.next() else {
        return Ok(());
    };
    let Some(version) = parts.next() else {
        return Err(MgError::Other(format!(
            "go.mod require without version: '{body}'"
        )));
    };
    // Quoted module paths are legal go.mod (`require "gopkg.in/check.v1" vX`
    // — gopkg.in/yaml.v3 does this) — strip the quotes or every downstream
    // URL carries them and 404s.
    // (Path module có quote là hợp lệ trong go.mod — cắt quote.)
    file.requires.push(GoRequire {
        path: path.trim_matches('"').to_string(),
        version: version.trim_start_matches('v').to_string(),
        indirect,
    });
    Ok(())
}

fn push_replace(file: &mut GoModFile, body: &str) -> MgResult<()> {
    let Some((old, new)) = body.split_once("=>") else {
        return Err(MgError::Other(format!(
            "go.mod replace missing '=>': '{body}'"
        )));
    };
    let mut old_parts = old.split_whitespace();
    let Some(old_path) = old_parts.next() else {
        return Err(MgError::Other(format!(
            "go.mod replace without old path: '{body}'"
        )));
    };
    let old_version = old_parts
        .next()
        .map(|v| v.trim_start_matches('v').to_string());
    let mut new_parts = new.split_whitespace();
    let Some(new_path) = new_parts.next() else {
        return Err(MgError::Other(format!(
            "go.mod replace without new target: '{body}'"
        )));
    };
    let new_version = new_parts
        .next()
        .map(|v| v.trim_start_matches('v').to_string());
    // Quoted paths are legal go.mod — strip like push_require.
    // (Path có quote là hợp lệ — cắt như push_require.)
    file.replaces.push(GoReplace {
        old_path: old_path.trim_matches('"').to_string(),
        old_version,
        new_path: new_path.trim_matches('"').to_string(),
        new_version,
    });
    Ok(())
}

fn push_exclude(file: &mut GoModFile, body: &str) -> MgResult<()> {
    let mut parts = body.split_whitespace();
    let Some(path) = parts.next() else {
        return Ok(());
    };
    let Some(version) = parts.next() else {
        return Err(MgError::Other(format!(
            "go.mod exclude without version: '{body}'"
        )));
    };
    file.excludes.push((
        path.trim_matches('"').to_string(),
        version.trim_start_matches('v').to_string(),
    ));
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
