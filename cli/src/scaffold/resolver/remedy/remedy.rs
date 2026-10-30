//! Remedy guidance for a scaffold layer the configured registry could not serve.
//! Hướng xử lý khi registry đã cấu hình không cung cấp được scaffold layer.
//!
//! MagiCore has no `mgc template` subcommand, so guidance must never name one.
//! After the local-generator work, every core with an adapter generator
//! (`game`, `iot`, `clo`) scaffolds fully offline; a layer miss on those cores
//! means the request itself is wrong (framework, engine or target), not that a
//! template needs fetching. For every core the honest remedies are an existing
//! project (`--from-local <path>`), the framework's own initializer, and the
//! registry configuration actually in effect.
//! MagiCore không có subcommand `mgc template`, nên hướng dẫn không bao giờ được
//! nêu lệnh đó. Sau phần local generator, mọi core có generator adapter (`game`,
//! `iot`, `clo`) scaffold được hoàn toàn offline; thiếu layer ở các core đó nghĩa
//! là request sai (framework/engine/đích), không phải cần tải template. Với mọi
//! core, hướng xử lý trung thực là project có sẵn (`--from-local <path>`),
//! initializer chính thức của framework, và cấu hình registry đang có hiệu lực.

/// Where the registry URL comes from, in the order `select_registry` applies.
/// Nguồn URL registry theo đúng thứ tự `select_registry` áp dụng.
pub const REGISTRY_CONFIG_SOURCES: &str = "`--registry <url>`, the MGC_NPM_REGISTRY environment variable, or the `registry=` entry in .npmrc";

/// Cores whose adapter generator writes the project offline, no registry needed.
/// Core có generator adapter ghi project offline, không cần registry.
pub const OFFLINE_GENERATOR_CORES: &[&str] = &["game", "iot", "clo"];

/// Lines a missing-layer diagnostic appends after the failed sources.
/// Các dòng diagnostic thiếu layer thêm sau danh sách nguồn đã thử.
pub fn missing_layer_next_steps(core: &str) -> String {
    let mut steps = String::new();
    if OFFLINE_GENERATOR_CORES.contains(&core) {
        steps.push_str(&format!(
            "- The '{core}' core scaffolds offline from its built-in generator: re-run \
             `mgc create-{core}` with a framework that generator supports (check \
             `mgc create-{core} --help`) instead of relying on the template registry\n"
        ));
    }
    steps.push_str(
        "- Reuse an existing project as the source: `mgc create-<core> <framework> <name> --from-local <path>`\n",
    );
    steps.push_str(
        "- Or scaffold with the framework's official initializer directly, then open the project with MagiCore\n",
    );
    steps.push_str(&format!("- Registry in effect: {REGISTRY_CONFIG_SOURCES}"));
    steps
}

/// Whether a user-facing diagnostic names a subcommand that does not exist.
///
/// `mgc template` is a module path, never a command. Only the form a user could
/// actually type counts — `mgc template` followed by an argument in the plain
/// ASCII word style of a clap subcommand (lowercase letters, digits, dashes) —
/// so prose such as "the mgc template module" outside the word boundary does
/// not trip the check.
/// Diagnostic hiển thị có nêu subcommand không tồn tại hay không. `mgc template`
/// là đường dẫn module, không phải lệnh. Chỉ tính dạng người dùng gõ được —
/// `mgc template` theo sau là một từ kiểu subcommand clap (chữ thường, số, gạch)
/// — nên câu văn như "the mgc template module" không làm sai kiểm tra.
#[cfg(test)]
pub(crate) fn names_nonexistent_subcommand(message: &str) -> bool {
    const INVOCATION: &str = "mgc template ";
    message.match_indices(INVOCATION).any(|(index, _)| {
        let rest = &message[index + INVOCATION.len()..];
        let word: String = rest
            .chars()
            .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-')
            .collect();
        // A verb-shaped word followed by whitespace (or end) is an invocation;
        // "template module" reads as a noun phrase and is ignored.
        // (Từ dạng verb theo sau là khoảng trắng (hoặc hết chuỗi) là lời gọi;
        // "template module" là cụm danh từ nên bỏ qua.)
        !word.is_empty() && !matches!(word.as_str(), "module" | "layer" | "layers")
    })
}
