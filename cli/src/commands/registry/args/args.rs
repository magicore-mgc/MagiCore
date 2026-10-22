//! Clap argument and subcommand contract for `mgc registry`.
//! Hợp đồng argument và subcommand Clap của `mgc registry`.

use clap::{Args, Subcommand};

/// Default local registry URL for user management.
/// URL registry cục bộ mặc định cho quản lý user.
const DEFAULT_REGISTRY: &str = "http://127.0.0.1:4315";

#[derive(Args, Debug, Clone)]
pub struct RegistryArgs {
    #[command(subcommand)]
    pub cmd: RegistryCmd,
}

#[derive(Subcommand, Debug, Clone)]
pub enum RegistryCmd {
    /// Start the private registry server (1 process, /npm + /v2)
    Serve {
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long, default_value = "4315")]
        port: u16,
        #[arg(long, default_value = "./data/registry")]
        store_dir: String,
        #[arg(long, env = "MAGICORE_REGISTRY_ADMIN_TOKEN")]
        admin_token: Option<String>,
        #[arg(long, default_value = "104857600")]
        max_body_size: usize,
        #[arg(long, default_value = "0", help = "rate limit (req/s/IP), 0 = off")]
        rate_limit: usize,
        #[arg(long, help = "upstream registry URL — GET-miss proxy + cache (ITEM 4)")]
        upstream: Option<String>,
        #[arg(
            long,
            help = "blob storage: \"local\" or \"s3://bucket/prefix\" (ITEM 5)"
        )]
        storage: Option<String>,
        #[arg(long = "oidc-issuer", env = "MAGICORE_REGISTRY_OIDC_ISSUER")]
        oidc_issuers: Vec<String>,
        #[arg(long, env = "MAGICORE_REGISTRY_OIDC_AUDIENCE")]
        oidc_audience: Option<String>,
        /// Comma-separated peer IPs of TLS proxies allowed to assert X-Forwarded-Proto.
        /// Danh sách IP peer proxy TLS được phép xác nhận X-Forwarded-Proto, phân tách bằng dấu phẩy.
        #[arg(
            long,
            env = "MAGICORE_REGISTRY_TRUSTED_PROXY_IPS",
            value_delimiter = ','
        )]
        trusted_proxy_ips: Vec<std::net::IpAddr>,
    },
    /// Bind a package to a trusted CI repository (requires admin token).
    /// Ràng buộc package với repository CI đáng tin (cần admin token).
    Trust {
        package: String,
        repository: String,
        #[arg(long, value_parser = ["npm", "pypi", "oci"], default_value = "npm")]
        protocol: String,
        #[arg(long, default_value = DEFAULT_REGISTRY)]
        registry: String,
        #[arg(long, env = "MAGICORE_REGISTRY_ADMIN_TOKEN")]
        admin_token: Option<String>,
    },
    /// Manage registry users
    User {
        #[command(subcommand)]
        cmd: UserCmd,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum UserCmd {
    /// Create a user and save the registry token
    Add {
        name: String,
        #[arg(long, help = "read password from stdin; otherwise prompt without echo")]
        password_stdin: bool,
        #[arg(long = "scope", help = "package scope patterns, vd: @magicore/*")]
        scopes: Vec<String>,
        #[arg(
            long,
            default_value = "publisher",
            help = "role: viewer|publisher|admin (ITEM 6)"
        )]
        role: String,
        #[arg(long, default_value = DEFAULT_REGISTRY)]
        registry: String,
    },
    /// Delete a user (requires admin token)
    Rm {
        name: String,
        #[arg(long, default_value = DEFAULT_REGISTRY)]
        registry: String,
        #[arg(long, env = "MAGICORE_REGISTRY_ADMIN_TOKEN")]
        admin_token: Option<String>,
    },
    /// Revoke an access token (requires admin token)
    Revoke {
        token: String,
        #[arg(long, default_value = DEFAULT_REGISTRY)]
        registry: String,
        #[arg(long, env = "MAGICORE_REGISTRY_ADMIN_TOKEN")]
        admin_token: Option<String>,
    },
}
