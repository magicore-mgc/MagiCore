// Strict audit semver range validation. / Kiểm tra semver range nghiêm ngặt cho audit.
use mgc_types::Version;

/// Audit-grade semver range validation: every `||` alternative must be
/// an operator + parseable version (`*` allowed). The general resolver
/// tolerates loose ranges; an AUDIT range that cannot be interpreted
/// must fail closed instead of matching nothing (which would hide real
/// findings behind malformed data from the registry).
/// Kiểm tra range semver cấp audit: mỗi alternative `||` phải là toán
/// tử + version parse được (cho phép `*`). Resolver chung dung thứ range
/// lỏng lẻo; range AUDIT không diễn giải được phải fail-closed thay vì
/// khớp rỗng (đề phòng giấu finding thật sau dữ liệu registry rác).
pub(in crate::audit) fn validate_semver_range(range: &str) -> Result<(), String> {
    for alt in range.split("||").map(str::trim) {
        if alt.is_empty() {
            return Err(format!("empty alternative in range '{range}'"));
        }
        if alt == "*" {
            continue;
        }
        // Tokens may be operator-only (">="), version-only ("4.17.21"),
        // or compact glued forms ("<4.17.21", "^1.2.3" — npm style).
        // Token có thể là riêng toán tử (">="), riêng version ("4.17.21"),
        // hoặc dạng dính gọn ("<4.17.21", "^1.2.3" — kiểu npm).
        let mut tokens = alt.split_whitespace().peekable();
        while let Some(token) = tokens.next() {
            if matches!(token, ">=" | "<=" | ">" | "<" | "=" | "==" | "^" | "~") {
                let Some(ver) = tokens.next() else {
                    return Err(format!(
                        "operator '{token}' in '{alt}' of range '{range}' has no version"
                    ));
                };
                if Version::parse(ver).is_err() {
                    return Err(format!(
                        "version '{ver}' in '{alt}' of range '{range}' is not parseable semver"
                    ));
                }
            } else {
                // Compact form: strip glued operators, then parse.
                // Dạng dính: bóc toán tử dính liền, rồi parse.
                let stripped = token.trim_start_matches(['>', '<', '=', '^', '~']);
                if Version::parse(stripped).is_err() {
                    return Err(format!(
                        "token '{token}' in '{alt}' of range '{range}' is not a parseable semver version"
                    ));
                }
            }
        }
    }
    Ok(())
}
