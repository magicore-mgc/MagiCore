//! Dispatch registry CLI actions to their feature owners.
//! Điều phối lệnh registry tới các owner chức năng.

use anyhow::Result;
use tracing_subscriber::EnvFilter;

use super::args::{RegistryArgs, RegistryCmd, UserCmd};

pub async fn run(args: RegistryArgs, core: Option<&str>) -> Result<()> {
    match args.cmd {
        RegistryCmd::Serve {
            host,
            port,
            store_dir,
            admin_token,
            max_body_size,
            rate_limit,
            upstream,
            storage,
            oidc_issuers,
            oidc_audience,
            trusted_proxy_ips,
            cors_origins,
        } => {
            let _ = tracing_subscriber::fmt()
                .with_env_filter(
                    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
                )
                .try_init(); // main.rs đã init global — không panic nếu set rồi
            mgc_registry_server::serve(mgc_registry_server::RegistryServerConfig {
                host,
                port,
                store_dir,
                admin_token,
                max_body_size,
                rate_limit_rps: rate_limit,
                upstream,
                storage,
                oidc_issuers,
                oidc_audience,
                trusted_proxy_ips,
                cors_origins,
            })
            .await
        }
        RegistryCmd::Trust {
            package,
            repository,
            protocol,
            registry,
            admin_token,
        } => {
            let package = super::trusted_binding::effective_trusted_binding_package(
                &protocol, core, &package,
            )?;
            super::trusted_binding::trust_publisher(
                &package,
                &repository,
                &protocol,
                &registry,
                admin_token,
            )
            .await?;
            Ok(())
        }
        RegistryCmd::User { cmd } => match cmd {
            UserCmd::Add {
                name,
                password_stdin,
                scopes,
                role,
                registry,
            } => {
                super::user_management::user_add(&name, password_stdin, &scopes, &role, &registry)
                    .await?;
                Ok(())
            }
            UserCmd::Rm {
                name,
                registry,
                admin_token,
            } => {
                super::user_management::user_rm(&name, &registry, admin_token).await?;
                Ok(())
            }
            UserCmd::Revoke {
                token,
                registry,
                admin_token,
            } => {
                super::user_management::revoke_token(&token, &registry, admin_token).await?;
                Ok(())
            }
        },
    }
}
