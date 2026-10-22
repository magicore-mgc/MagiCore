//! Builds Maven Quarkus or Spring Boot dev commands.
//! Dựng lệnh dev Maven Quarkus hoặc Spring Boot.

use anyhow::Result;
use std::ffi::OsString;
use std::path::PathBuf;

use super::super::super::types::DevLaunch;
use super::super::environment::env_host_port_pairs;
use super::super::types::NativeDevContext;

pub(in crate::commands::core::web::implementation::dev::native_launch) fn infer_java_dev_launch(
    context: &NativeDevContext<'_>,
) -> Result<Option<DevLaunch>> {
    let pom_path = context.project_root.join("pom.xml");
    if !pom_path.exists() {
        return Ok(None);
    }
    let pom = std::fs::read_to_string(pom_path).unwrap_or_default();
    let host = context.host.unwrap_or("localhost");
    if pom.contains("quarkus.platform") || pom.contains("io.quarkus") {
        let mut args = vec![
            OsString::from("quarkus:dev"),
            OsString::from(format!("-Dquarkus.http.host={host}")),
            OsString::from("-Dquarkus.analytics.disabled=true"),
        ];
        if let Some(port) = context.port {
            args.push(OsString::from(format!("-Dquarkus.http.port={port}")));
        }
        return Ok(Some(DevLaunch {
            program: PathBuf::from("mvn"),
            args,
            envs: env_host_port_pairs(context),
        }));
    }
    let host_arg = format!("--server.address={host}");
    let mut args = vec![OsString::from("spring-boot:run")];
    if let Some(port) = context.port {
        args.push(OsString::from(format!(
            "-Dspring-boot.run.arguments=--server.port={port} {host_arg}"
        )));
    } else {
        args.push(OsString::from(format!(
            "-Dspring-boot.run.arguments={host_arg}"
        )));
    }
    Ok(Some(DevLaunch {
        program: PathBuf::from("mvn"),
        args,
        envs: env_host_port_pairs(context),
    }))
}
