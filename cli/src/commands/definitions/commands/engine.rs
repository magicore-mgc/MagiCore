use crate::commands::definitions::ToolCmd;
// EngineCommands schema — khai báo nhóm lệnh CLI theo trách nhiệm.
use clap::Subcommand;

#[derive(Subcommand, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum EngineCommands {
    // ── Engine Commands (In-project, auto-detect core) ───────────────
    #[command(about = "Start the local development server", alias = "dev-web")]
    Dev {
        #[arg(long)]
        host: Option<String>,
        #[arg(long)]
        port: Option<u16>,
        #[arg(long, help = "Clear terminal on each reload")]
        clear: bool,
        /// Temporary rival-runtime compatibility lane (bun|deno) —
        /// explicit, warned on every spawn; the default path is the
        /// NATIVE MagiCore engine.
        /// Lane compat runtime đối thủ tạm thời — tường minh, cảnh báo
        /// mỗi lần spawn; đường mặc định là engine NATIVE MagiCore.
        #[arg(
            long,
            help = "Legacy flag; Bun/Deno execution is unsupported by the native engine"
        )]
        compat_runtime: Option<String>,
        /// Delegated runner lane (npm|pnpm|yarn|bun) — explicit opt-in,
        /// warned on every use; dependency/lifecycle operations are no
        /// longer fully owned by MagiCore.
        /// Lane runner ủy quyền — opt-in tường minh, cảnh báo mỗi lần
        /// dùng; MagiCore không còn sở hữu toàn phần dependency/lifecycle.
        #[arg(
            long = "compat-runner",
            value_name = "MANAGER",
            help = "Delegate execution to a qualified package-manager runner (npm|pnpm|yarn|bun); warned + audit-logged"
        )]
        compat_runner: Option<String>,
    },
    #[command(about = "Run a script defined in package.json")]
    Run {
        #[arg(required = true)]
        script: String,
        #[arg(last = true)]
        args: Vec<String>,
        /// Temporary rival-runtime compatibility lane (bun|deno).
        #[arg(
            long,
            help = "Legacy flag; Bun/Deno execution is unsupported by the native engine"
        )]
        compat_runtime: Option<String>,
        /// Delegated runner lane (npm|pnpm|yarn|bun) — explicit opt-in,
        /// warned on every use; dependency/lifecycle operations are no
        /// longer fully owned by MagiCore.
        /// Lane runner ủy quyền — opt-in tường minh, cảnh báo mỗi lần
        /// dùng; MagiCore không còn sở hữu toàn phần dependency/lifecycle.
        #[arg(
            long = "compat-runner",
            value_name = "MANAGER",
            help = "Delegate execution to a qualified package-manager runner (npm|pnpm|yarn|bun); warned + audit-logged"
        )]
        compat_runner: Option<String>,
    },
    #[command(about = "Run tests with a detected native runner or the project test script")]
    Test {
        #[arg(last = true, help = "Arguments passed to test runner")]
        args: Vec<String>,
        /// Temporary rival-runtime compatibility lane (bun|deno).
        #[arg(
            long,
            help = "Legacy flag; Bun/Deno execution is unsupported by the native engine"
        )]
        compat_runtime: Option<String>,
        /// Delegated runner lane (npm|pnpm|yarn|bun) — explicit opt-in,
        /// warned on every use; dependency/lifecycle operations are no
        /// longer fully owned by MagiCore.
        /// Lane runner ủy quyền — opt-in tường minh, cảnh báo mỗi lần
        /// dùng; MagiCore không còn sở hữu toàn phần dependency/lifecycle.
        #[arg(
            long = "compat-runner",
            value_name = "MANAGER",
            help = "Delegate execution to a qualified package-manager runner (npm|pnpm|yarn|bun); warned + audit-logged"
        )]
        compat_runner: Option<String>,
    },
    #[command(about = "Optimize project for hardware (runtime detection + adapter pattern)")]
    Optimizer {
        #[arg(long, help = "Overwrite existing optimized files")]
        force: bool,
    },
    #[command(about = "Build the project")]
    Build {
        #[arg(long, help = "Build target (e.g., native, browser, server)")]
        target: Option<String>,
        /// Temporary rival-runtime compatibility lane (bun|deno).
        #[arg(
            long,
            help = "Legacy flag; Bun/Deno execution is unsupported by the native engine"
        )]
        compat_runtime: Option<String>,
        /// Delegated runner lane (npm|pnpm|yarn|bun) — explicit opt-in,
        /// warned on every use; dependency/lifecycle operations are no
        /// longer fully owned by MagiCore.
        /// Lane runner ủy quyền — opt-in tường minh, cảnh báo mỗi lần
        /// dùng; MagiCore không còn sở hữu toàn phần dependency/lifecycle.
        #[arg(
            long = "compat-runner",
            value_name = "MANAGER",
            help = "Delegate execution to a qualified package-manager runner (npm|pnpm|yarn|bun); warned + audit-logged"
        )]
        compat_runner: Option<String>,
        /// Which engine compiles the project.
        /// Engine nào biên dịch project.
        #[arg(
            long,
            value_name = "ENGINE",
            value_parser = ["auto", "magi", "framework"],
            help = "Engine: auto (by framework capability), magi (in-process bundler), framework (the framework's own program)"
        )]
        engine: Option<String>,
        /// Ship the output even when the scan found a blocking issue.
        /// Vẫn phát hành output dù scan tìm thấy vấn đề chặn.
        #[arg(
            long,
            help = "Do not fail the build when the output scan reports a blocking finding"
        )]
        allow_output_warnings: bool,
        /// Keep native dependency builds off the network.
        /// Giữ build dependency native không dùng mạng.
        #[arg(
            long,
            help = "Build native dependencies (cargo, gradle) without network access; the build fails if a dependency is not already installed"
        )]
        offline: bool,
    },
    #[command(about = "Verify an SBOM signature written by `mgc build` (fail-closed)")]
    VerifySbom {
        /// Artifact to verify; its sibling `<artifact>.sig` is read.
        /// Artifact cần verify; file `<artifact>.sig` cạnh nó được đọc.
        artifact: String,
        /// Pinned Ed25519 public key (base64) that must have produced the
        /// signature; copy it from `mgc trust list` for the key that signed.
        /// Public key Ed25519 đã pin (base64) phải là key tạo chữ ký; lấy từ
        /// `mgc trust list` cho đúng khoá đã ký.
        #[arg(long = "trust-key", value_name = "BASE64")]
        trust_key: String,
    },
    #[command(about = "Flash firmware to a device (IoT esp32)")]
    Flash {
        #[arg(long, help = "Board override (e.g., esp32c3, esp32s3)")]
        board: Option<String>,
        #[arg(long, help = "Skip cargo build, flash existing binary")]
        skip_build: bool,
    },
    #[command(
        about = "Deploy cloud infrastructure (native provider engines are not yet available)"
    )]
    Deploy {
        #[arg(
            long,
            help = "Legacy flag; no vendor CLI is invoked without a native provider engine"
        )]
        run: bool,
    },
    #[command(about = "Generate CI pipeline (github-actions workflows)")]
    CiGenerate,
    #[command(about = "Run verify chain (audit → test → build) per project core")]
    Verify,
    #[command(about = "Start the production server")]
    Start,
    #[command(about = "Execute a shell command in scope of a project")]
    Exec {
        #[arg(required = true)]
        command: String,
        #[arg(last = true)]
        args: Vec<String>,
        /// Delegated runner lane (npm|pnpm|yarn|bun) — explicit opt-in,
        /// warned on every use; dependency/lifecycle operations are no
        /// longer fully owned by MagiCore.
        /// Lane runner ủy quyền — opt-in tường minh, cảnh báo mỗi lần
        /// dùng; MagiCore không còn sở hữu toàn phần dependency/lifecycle.
        #[arg(
            long = "compat-runner",
            value_name = "MANAGER",
            help = "Delegate execution to a qualified package-manager runner (npm|pnpm|yarn|bun); warned + audit-logged"
        )]
        compat_runner: Option<String>,
    },
    #[command(about = "Download and execute a package without permanently installing it")]
    Dlx {
        #[arg(required = true)]
        package: String,
        /// Select an executable declared by the requested package.
        /// Chọn bin của package yêu cầu khi package cung cấp nhiều executable.
        #[arg(
            long,
            value_name = "NAME",
            help = "Select a declared executable when the package provides multiple binaries"
        )]
        bin: Option<String>,
        /// Explicitly run lifecycle scripts while installing into the temporary dlx project.
        /// Cho phép rõ ràng chạy lifecycle scripts trong project dlx tạm.
        #[arg(
            long,
            help = "Allow package lifecycle scripts during temporary installation"
        )]
        allow_scripts: bool,
        #[arg(last = true)]
        args: Vec<String>,
    },
    #[command(about = "Install and manage persistent user-scoped CLI tools")]
    Tool {
        #[command(subcommand)]
        cmd: ToolCmd,
    },
    #[command(about = "Inspect or clean MagiCore caches")]
    Cache {
        #[arg(value_parser = ["status", "clean", "prune"])]
        action: String,
        #[arg(long, default_value = "all", value_parser = ["all", "shared", "project", "build"])]
        target: String,
        #[arg(long, help = "Required for cache clean")]
        yes: bool,
        #[arg(long, help = "Preview cache prune without deleting files")]
        dry_run: bool,
        #[arg(
            long,
            help = "Prune as CI does: keep only package roots a live project references, drop tarballs and unlinked files"
        )]
        ci: bool,
    },
}
