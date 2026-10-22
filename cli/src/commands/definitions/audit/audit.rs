// AuditCmd argument schema — khai báo tham số cho audit.
use clap::Subcommand;

#[derive(Subcommand, Clone)]
pub enum AuditCmd {
    /// Verify registry provenance signatures and the package transparency chain.
    /// Xác minh chữ ký provenance registry và chuỗi minh bạch của package.
    Signatures {
        package: String,
        #[arg(long, value_parser = ["npm", "pypi", "oci"], default_value = "npm")]
        protocol: String,
        #[arg(long, env = "MGC_REGISTRY", default_value = "http://127.0.0.1:4315")]
        registry: String,
        #[arg(long, env = "MGC_NPM_TOKEN")]
        token: Option<String>,
        #[arg(long)]
        json: bool,
    },
}
