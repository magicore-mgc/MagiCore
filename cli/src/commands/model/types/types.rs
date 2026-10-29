// types owner — types owner implementation.
// Responsibility boundary — ranh giới trách nhiệm của owner.
use clap::{Args, Subcommand};

const DEFAULT_REGISTRY: &str = "http://127.0.0.1:4315";
#[derive(Args, Debug, Clone)]
pub struct ModelArgs {
    #[command(subcommand)]
    pub cmd: ModelCmd,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ModelCmd {
    /// Push model files to registry (one layer per file + manifest)
    Push {
        /// Model files/directories (directories are walked; one layer per file)
        #[arg(required = true)]
        paths: Vec<String>,
        /// Repo, e.g. ai/my-model
        #[arg(long, default_value = "ai/default")]
        repo: String,
        #[arg(long, default_value = "latest")]
        tag: String,
        #[arg(long, default_value = DEFAULT_REGISTRY)]
        registry: String,
        #[arg(long, env = "MAGICORE_REGISTRY_ADMIN_TOKEN")]
        token: Option<String>,
    },
    /// Pull model: `hf://org/model/file` / `oci://registry/repo:tag` (→ CAS store)
    /// or from the local registry (writes files as-is into the output dir).
    Pull {
        /// Repo: hf://org/model/file | oci://registry/repo:tag | ai/my-model (local registry)
        repo: String,
        #[arg(long, default_value = "latest")]
        tag: String,
        #[arg(long, default_value = DEFAULT_REGISTRY)]
        registry: String,
        #[arg(long, default_value = ".")]
        output: String,
        /// Immutable 40-hex Hugging Face commit revision (required for hf://).
        #[arg(long)]
        revision: Option<String>,
        /// Trusted SHA-256 of the exact Hugging Face artifact (required for hf://).
        #[arg(long)]
        sha256: Option<String>,
        /// Maximum Hugging Face download size in bytes (default: 64 GiB).
        #[arg(long)]
        max_bytes: Option<u64>,
        #[arg(long, env = "MAGICORE_REGISTRY_ADMIN_TOKEN")]
        token: Option<String>,
    },
    /// List: default is the registry catalog; `--local` = models in the CAS store
    List {
        #[arg(long)]
        local: bool,
        #[arg(long, default_value = DEFAULT_REGISTRY)]
        registry: String,
        #[arg(long, env = "MAGICORE_REGISTRY_ADMIN_TOKEN")]
        token: Option<String>,
    },
    /// Remove a local model from the CAS store (manifest + unreferenced blobs)
    Rm {
        /// Model name (matches manifest: org/model/file or repo:tag)
        name: String,
    },
    /// Measure inference latency per runtime for a pulled model (E-4).
    /// Đo độ trễ inference theo từng runtime cho model đã pull (E-4).
    ///
    /// This never trains or converts a model: it runs the model's runtime and
    /// reports measured wall-clock numbers, so a latency claim can cite real
    /// output instead of an estimate.
    /// (Lệnh này không train hay convert model: nó chạy runtime của model và
    /// báo số wall-clock đo được, để claim về độ trễ có thể dẫn chứng thay vì
    /// ước lượng.)
    Bench {
        /// Model name recorded in the local manifest (see `mgc model list --local`).
        /// Tên model trong manifest local (xem `mgc model list --local`).
        model: String,
        /// Model artifact path on disk.
        /// Đường dẫn file model trên đĩa.
        #[arg(long, value_name = "PATH")]
        path: String,
        /// Invocations per runtime; medians need several samples.
        /// Số lần gọi mỗi runtime; trung vị cần nhiều mẫu.
        #[arg(long, default_value_t = 5)]
        samples: u32,
        #[arg(long, help = "output JSON")]
        json: bool,
    },
    /// Quantize GGUF (native quantizer not yet implemented).
    Quantize {
        /// Path to the source model file (GGUF/ggml)
        path: String,
        /// Target quant: q4_k_m | q8_0 (awq needs a GPU toolchain — unsupported)
        #[arg(long, default_value = "q4_k_m")]
        target: String,
        /// Output path (default: <path>.<target>.gguf in the same directory)
        #[arg(long)]
        output: Option<String>,
    },
}
