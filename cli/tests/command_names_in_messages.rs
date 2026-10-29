//! User-facing messages must name a command that exists.
//!
//! Three separate wrong names shipped in user-facing text: `mgc optimize`
//! (real: `mgc optimizer`), `mgc ci generate` (real: `mgc ci-generate`) and
//! `mgc key generate` (no such command at all — the loader wants a keyring JSON
//! file). A message naming a nonexistent command sends the user to a shell error
//! right after the real command told them something useful.
//! (Ba tên lệnh sai từng xuất hiện trong text cho người dùng. Message nêu lệnh
//! không tồn tại đẩy người dùng tới lỗi shell ngay sau khi vừa được báo điều hữu
//! ích.)

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .to_path_buf()
}

fn collect_rs(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|name| name == "target") {
                continue;
            }
            collect_rs(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

/// Subcommands the binary actually accepts.
fn real_subcommands() -> BTreeSet<String> {
    let binary = env!("CARGO_BIN_EXE_mgc");
    let output = Command::new(binary)
        .arg("--help")
        .output()
        .expect("run mgc --help");
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| line.starts_with("  "))
        .filter_map(|line| line.split_whitespace().next())
        .map(str::to_string)
        .collect()
}

/// No `mgc <word>` in production source may name a subcommand that is absent
/// from the binary's own help output.
///
/// Multi-word forms are checked on their first word (`mgc ci generate` fails
/// because `ci` is not a subcommand). Placeholders such as `mgc create-<game>`
/// are skipped: the angle bracket marks them as patterns, not commands.
/// (Mọi `mgc <từ>` trong source production phải là subcommand thật. Dạng nhiều
/// chữ kiểm ở chữ đầu. Placeholder `mgc create-<game>` được bỏ qua.)
#[test]
fn no_message_names_a_nonexistent_subcommand() {
    let root = repo_root();
    let real = real_subcommands();
    assert!(
        real.len() > 50,
        "the help output parse produced too few subcommands ({}); the guard would be blind",
        real.len()
    );

    let mut files = Vec::new();
    for dir in ["cli/src", "core/crates", "adapters"] {
        collect_rs(&root.join(dir), &mut files);
    }

    let mut offenders = BTreeSet::new();
    for file in files {
        let rel = file
            .strip_prefix(&root)
            .unwrap_or(&file)
            .display()
            .to_string();
        if rel.contains("/test/") || rel.contains("/test.rs") {
            continue;
        }
        let Ok(source) = std::fs::read_to_string(&file) else {
            continue;
        };
        for (index, line) in source.lines().enumerate() {
            // A doc comment may legally talk ABOUT a nonexistent command name
            // ("there is no `mgc template`"); that explains absence instead of
            // instructing the user.
            // (Doc comment có thể nói VỀ tên lệnh không tồn tại để giải thích sự
            // vắng mặt, không bảo người dùng chạy nó.)
            let trimmed = line.trim_start();
            let explains_absence = trimmed.starts_with("//!")
                || trimmed.starts_with("///")
                || trimmed.starts_with("//");
            // Only a COMMAND reference can mislead: one the user is told to type,
            // i.e. written in backticks or as an `mgc ...` invocation in prose
            // that starts a command. "mgc owns the manifest" is English, not a
            // command name — matching it produced 13 false positives.
            // (Chỉ tham chiếu LỆNH mới gây nhầm: lệnh người dùng được bảo gõ, tức
            // nằm trong backtick. "mgc owns the manifest" là văn xuôi — khớp nó tạo
            // 13 báo động giả.)
            let mut rest = line;
            while let Some(pos) = rest.find("`mgc ") {
                let after = &rest[pos + 5..];
                let word: String = after
                    .chars()
                    .take_while(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-')
                    .collect();
                rest = &rest[pos + 5..];
                if word.is_empty() {
                    continue;
                }
                // A placeholder such as `create-<core>` / `create-{core}` is a
                // pattern, not a command: the `-` is followed by a bracket.
                // (Placeholder như `create-<core>` / `create-{core}` là pattern,
                // không phải lệnh.)
                // `take_while` stops at the bracket, so the tail for
                // `create-{core}` is `{core}` and for `create-<core>` is `-<core>`.
                // (take_while dừng ở dấu ngoặc nên tail là `{core}` hoặc `-<core>`.)
                let tail = &after[word.len()..];
                if tail.starts_with('{') || tail.starts_with("-<") || tail.starts_with("-{") {
                    continue;
                }
                // A trailing `-` means the name was cut by a bracket placeholder.
                if word.ends_with('-') {
                    continue;
                }
                // Prose such as "`mgc` owns" has no following word.
                if word.chars().next().is_some_and(|c| !c.is_ascii_lowercase()) {
                    continue;
                }
                if explains_absence {
                    continue;
                }
                if !real.contains(&word) {
                    offenders.insert(format!("{rel}:{} -> mgc {word}", index + 1));
                }
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "these lines name a `mgc` subcommand that does not exist:\n  {}",
        offenders.into_iter().collect::<Vec<_>>().join("\n  ")
    );
}

/// The canonical names used by the messages resolve.
/// (Các tên chuẩn được message dùng đều tồn tại.)
#[test]
fn the_canonical_command_names_resolve() {
    let binary = env!("CARGO_BIN_EXE_mgc");
    for command in [
        "optimizer",
        "bench",
        "ci-generate",
        "verify",
        "deploy",
        "publish",
    ] {
        let output = Command::new(binary)
            .args([command, "--help"])
            .output()
            .expect("run mgc --help");
        assert!(
            output.status.success(),
            "`mgc {command} --help` must succeed; messages may name it"
        );
    }
}
