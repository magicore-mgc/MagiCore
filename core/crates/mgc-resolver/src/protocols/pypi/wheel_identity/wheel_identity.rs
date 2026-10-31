use mgc_types::{MgError, MgResult};

/// Confirm wheel metadata identity before its files enter Python's import path.
///
/// Keys are matched case-insensitively and the Name/Version headers are read
/// with the RFC 822 continuation form Python writes (an indented next line
/// continues the previous value), never by splitting on the first colon alone.
/// Peppercorn-generated distributions normalize `.devN`/`.postN` versions
/// differently from `pyproject.toml`, so comparing only the release segments
/// rejected legitimate wheels; the comparison below normalizes both sides per
/// PEP 440 instead.
/// (Xác nhận identity trong METADATA khớp tên/version trong lock. So khớp theo
/// chuẩn PEP 440, gồm cả `.devN`/`.postN`, để không từ chối nhầm wheel hợp lệ.)
pub(in crate::protocols::pypi) fn verify_wheel_identity(
    metadata: &[u8],
    expected_name: &str,
    expected_version: &str,
) -> MgResult<()> {
    let metadata = std::str::from_utf8(metadata)
        .map_err(|_| MgError::Integrity("Python wheel METADATA is not UTF-8".to_string()))?;
    let (name, version) = read_core_metadata(metadata)?;
    let name = name
        .ok_or_else(|| MgError::Integrity("Python wheel METADATA has no Name field".to_string()))?;
    let version = version.ok_or_else(|| {
        MgError::Integrity("Python wheel METADATA has no Version field".to_string())
    })?;
    // Two `None`s must never compare equal: an unparsable version fails
    // closed instead of matching nothing.
    // (Hai `None` không được coi là khớp; version không parse được thì từ chối.)
    let identity_matches = match (
        normalize_pep440_version(&version),
        normalize_pep440_version(expected_version),
    ) {
        (Some(actual), Some(expected)) => actual == expected,
        _ => false,
    };
    if normalize_pep503_name(&name) != normalize_pep503_name(expected_name) || !identity_matches {
        return Err(MgError::Integrity(format!(
            "Python wheel METADATA identity does not match locked {}@{} (wheel declares {}@{})",
            expected_name, expected_version, name, version
        )));
    }
    Ok(())
}

/// First occurrence of a repeated core field is an integrity failure, not a
/// last-wins parse: a wheel that declares two identities is ambiguous.
/// (Trường core lặp lại là lỗi integrity, không phải chọn giá trị cuối.)
fn set_once(slot: &mut Option<String>, value: &str, field: &'static str) -> MgResult<()> {
    if slot.replace(value.to_string()).is_some() {
        return Err(MgError::Integrity(format!(
            "Python wheel METADATA repeats {field}"
        )));
    }
    Ok(())
}

/// Read `Name` and `Version` from the RFC 822 header block.
/// (Đọc `Name` và `Version` từ khối header RFC 822.)
fn read_core_metadata(metadata: &str) -> MgResult<(Option<String>, Option<String>)> {
    let mut name = None;
    let mut version = None;
    // Pending `(field, value)` awaiting a possible continuation line.
    // (Trường đang chờ dòng nối tiếp.)
    let mut pending: Option<(&'static str, String)> = None;
    fn flush(
        pending: &mut Option<(&'static str, String)>,
        name: &mut Option<String>,
        version: &mut Option<String>,
    ) -> MgResult<()> {
        let Some((field, value)) = pending.take() else {
            return Ok(());
        };
        let value = value.trim();
        match field {
            "Name" => set_once(name, value, "Name"),
            _ => set_once(version, value, "Version"),
        }
    }
    for line in metadata.lines() {
        if line.is_empty() {
            break;
        }
        // A folded header continues the previous value; check it first so an
        // indented `:` inside the value is never re-parsed as a new field.
        // (Dòng nối tiếp được kiểm trước để `:` trong giá trị không bị parse lại.)
        if line.starts_with([' ', '\t']) {
            if let Some((_, value)) = pending.as_mut() {
                value.push_str(line.trim());
            }
            continue;
        }
        flush(&mut pending, &mut name, &mut version)?;
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let field = match key.trim().to_ascii_lowercase().as_str() {
            "name" => "Name",
            "version" => "Version",
            _ => continue,
        };
        pending = Some((field, value.to_string()));
    }
    flush(&mut pending, &mut name, &mut version)?;
    Ok((name, version))
}

/// PEP 503 distribution name: lowercase, runs of `-`, `_`, `.` collapse to `-`.
/// (Tên distribution theo PEP 503: lowercase, gộp `-`/`_`/`.` thành `-`.)
fn normalize_pep503_name(value: &str) -> String {
    let mut normalized = String::with_capacity(value.len());
    let mut separator = false;
    for ch in value.chars() {
        if matches!(ch, '-' | '_' | '.') {
            separator = true;
            continue;
        }
        if separator && !normalized.is_empty() {
            normalized.push('-');
        }
        separator = false;
        normalized.extend(ch.to_lowercase());
    }
    normalized
}

/// Canonical PEP 440 spelling of a public version, or `None` for text PEP 440
/// does not define (the caller then refuses rather than guesses).
///
/// Returns the release segments plus the optional `pre`, `post`, `dev` and
/// local-label numbers; the caller renders them. Parsing never filters the
/// input first — a filtered copy both cost the original string and made the
/// empty parse look like a successful match.
/// (Chuẩn hóa version theo PEP 440 đầy đủ; không rõ → từ chối.)
fn parse_pep440_version(value: &str) -> Option<Pep440Parts> {
    /// Advance past any run of `.`, `-`, `_` separators.
    /// (Bỏ qua mọi chuỗi phân cách `.`, `-`, `_`.)
    fn skip_separators(value: &[u8], index: &mut usize) {
        while *index < value.len() && matches!(value[*index], b'.' | b'-' | b'_') {
            *index += 1;
        }
    }

    /// Read a digit run at `index`; an empty run is `None`.
    /// (Đọc chuỗi chữ số tại index; chuỗi rỗng là `None`.)
    fn take_number(value: &[u8], index: &mut usize) -> Option<u64> {
        let start = *index;
        while *index < value.len() && value[*index].is_ascii_digit() {
            *index += 1;
        }
        if *index == start {
            return None;
        }
        std::str::from_utf8(&value[start..*index])
            .ok()?
            .parse()
            .ok()
    }

    /// Match one keyword (with optional leading separators) at `index`.
    /// (Khớp một từ khóa, có thể kèm phân cách phía trước.)
    fn take_word<'a>(value: &[u8], index: &mut usize, words: &[&'a str]) -> Option<&'a str> {
        skip_separators(value, index);
        let rest = value.get(*index..)?;
        for word in words {
            if rest.starts_with(word.as_bytes()) {
                *index += word.len();
                return Some(word);
            }
        }
        None
    }

    let trimmed = value.trim();
    let trimmed = trimmed
        .strip_prefix('v')
        .or_else(|| trimmed.strip_prefix('V'))
        .unwrap_or(trimmed);
    if trimmed.is_empty() || !trimmed.is_ascii() {
        return None;
    }
    let lower = trimmed.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    let mut index = 0usize;

    let mut epoch = 0u64;
    let digits_end = bytes
        .iter()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    if digits_end > 0 && bytes.get(digits_end) == Some(&b'!') {
        epoch = std::str::from_utf8(&bytes[..digits_end])
            .ok()?
            .parse()
            .ok()?;
        index = digits_end + 1;
    }

    // Release: `N(.N)*`; trailing zero segments are not significant.
    // (Release: `N(.N)*`; các đoạn 0 ở cuối không có ý nghĩa.)
    let mut release = vec![take_number(bytes, &mut index)?];
    while bytes.get(index) == Some(&b'.') && bytes.get(index + 1).is_some_and(u8::is_ascii_digit) {
        index += 1;
        release.push(take_number(bytes, &mut index)?);
    }
    while release.last() == Some(&0) && release.len() > 1 {
        release.pop();
    }

    // Fixed PEP 440 order: optional pre, then post, then dev.
    // (Thứ tự PEP 440 cố định: pre → post → dev.)
    let mut pre = None;
    if index < bytes.len() {
        for (prefix, label) in [
            ("alpha", "a"),
            ("beta", "b"),
            ("preview", "rc"),
            ("pre", "rc"),
            ("a", "a"),
            ("b", "b"),
            ("c", "rc"),
            ("rc", "rc"),
        ] {
            if bytes[index..].starts_with(prefix.as_bytes()) {
                index += prefix.len();
                skip_separators(bytes, &mut index);
                pre = Some((label, take_number(bytes, &mut index).unwrap_or(0)));
                break;
            }
        }
    }
    let mut post = None;
    if take_word(bytes, &mut index, &["post", "rev", "r"]).is_some() {
        skip_separators(bytes, &mut index);
        post = Some(take_number(bytes, &mut index).unwrap_or(0));
    }
    let mut dev = None;
    if take_word(bytes, &mut index, &["dev"]).is_some() {
        skip_separators(bytes, &mut index);
        dev = Some(take_number(bytes, &mut index).unwrap_or(0));
    }

    let mut local = None;
    if bytes.get(index) == Some(&b'+') {
        index += 1;
        let mut segments = Vec::new();
        loop {
            let start = index;
            while index < bytes.len() && !matches!(bytes[index], b'.' | b'-' | b'_') {
                index += 1;
            }
            if index == start {
                return None;
            }
            let segment = std::str::from_utf8(&bytes[start..index]).ok()?;
            segments.push(match segment.parse::<u64>() {
                Ok(number) => number.to_string(),
                Err(_) => segment.to_string(),
            });
            if index == bytes.len() || !matches!(bytes[index], b'.' | b'-' | b'_') {
                break;
            }
            index += 1;
        }
        local = Some(segments.join("."));
    }

    if index != bytes.len() {
        return None;
    }
    Some(Pep440Parts {
        epoch,
        release,
        pre,
        post,
        dev,
        local,
    })
}

/// Parsed PEP 440 version components, all numbers already normalized.
/// (Các thành phần version PEP 440 đã chuẩn hóa.)
struct Pep440Parts {
    epoch: u64,
    release: Vec<u64>,
    pre: Option<(&'static str, u64)>,
    post: Option<u64>,
    dev: Option<u64>,
    local: Option<String>,
}

impl Pep440Parts {
    /// Canonical rendering; two versions are equivalent iff these are equal.
    /// (Dạng chuẩn tắc; hai version tương đương khi chuỗi này trùng nhau.)
    fn canonical(&self) -> String {
        let mut canonical = String::new();
        if self.epoch != 0 {
            canonical.push_str(&format!("{}!", self.epoch));
        }
        canonical.push_str(
            &self
                .release
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join("."),
        );
        if let Some((label, number)) = self.pre {
            canonical.push_str(&format!("{label}{number}"));
        }
        if let Some(number) = self.post {
            canonical.push_str(&format!(".post{number}"));
        }
        if let Some(number) = self.dev {
            canonical.push_str(&format!(".dev{number}"));
        }
        if let Some(local) = &self.local {
            canonical.push('+');
            canonical.push_str(local);
        }
        canonical
    }
}

/// Canonical PEP 440 spelling of a public version, or `None` when the text is
/// not a version PEP 440 defines.
/// (Dạng chuẩn tắc của version công khai theo PEP 440.)
fn normalize_pep440_version(value: &str) -> Option<String> {
    parse_pep440_version(value).map(|parts| parts.canonical())
}
