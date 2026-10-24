//! `mgc version` — print the CLI version.
//! `mgc version` — in phiên bản CLI.
//!
//! `--version` already exists as a clap flag; this subcommand exists so scripts
//! and CI can query the version through the same command tree as everything
//! else, and optionally as machine-readable JSON.
//! `--version` đã có dạng cờ clap; subcommand này để script/CI hỏi version qua
//! cùng cây lệnh, và tùy chọn xuất JSON cho máy đọc.

/// Print the CLI version, plain or as JSON.
/// In phiên bản CLI, dạng thường hoặc JSON.
pub fn run(json: bool) -> anyhow::Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    if json {
        println!("{{\"name\":\"mgc\",\"version\":\"{version}\"}}");
    } else {
        println!("mgc {version}");
    }
    Ok(())
}
