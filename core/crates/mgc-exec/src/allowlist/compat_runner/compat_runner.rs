/// True when `cmd` is the declared compat runner itself (basename match).
/// The subtree it spawns is governed by the compat audit-only monitor, not
/// by this name check.
/// (Đúng khi `cmd` chính là runner compat đã khai báo (khớp basename).
/// Subtree nó spawn chịu monitor audit-only compat, không chịu name check.)
pub fn compat_runner_allows(cmd: &str, runner: Option<&str>) -> bool {
    let Some(runner) = runner else {
        return false;
    };
    let base = cmd
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(cmd)
        .to_ascii_lowercase();
    let base = base
        .strip_suffix(".exe")
        .or_else(|| base.strip_suffix(".cmd"))
        .unwrap_or(&base);
    base == runner
}
