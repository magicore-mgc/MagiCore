use super::super::*;
use super::CloudBuildError;

/// Cloud build (06 §4): cdk synth (qua node_modules/.bin — npm-format, như web pattern),
/// pulumi preview, terraform plan — đều là dry-run (không ghi cloud state).
/// Fail-closed: toolchain thiếu → cảnh báo, không fail project.
#[cfg(feature = "clo")]
pub(in crate::commands::build::implementation) fn cloud_local_executable_exists(
    root: &Path,
    bin_name: &str,
    windows: bool,
) -> bool {
    let bin_dir = root.join("node_modules").join(".bin");
    let bin = bin_dir.join(bin_name);
    // Windows npm creates a .cmd shim; Unix uses the extensionless entry.
    // npm trên Windows tạo shim .cmd; Unix dùng entry không có phần mở rộng.
    bin.exists() || (windows && bin.with_extension("cmd").exists())
}

#[cfg(feature = "clo")]
pub(in crate::commands::build::implementation) fn cloud_cdk_synth_env(
    local_bin: &Path,
    home: &Path,
) -> Result<Vec<(String, String)>> {
    Ok(vec![
        (
            "PATH".to_string(),
            prepend_path(local_bin)?.to_string_lossy().to_string(),
        ),
        ("HOME".to_string(), home.to_string_lossy().to_string()),
        (
            "USERPROFILE".to_string(),
            home.to_string_lossy().to_string(),
        ),
        // CDK telemetry deliberately runs in a short-lived background child;
        // disable it so the process-tree guard sees only synth work.
        // Telemetry CDK chủ động chạy bằng process nền ngắn hạn; tắt nó để
        // guard cây process chỉ theo dõi đúng công việc synth.
        // Source: https://docs.aws.amazon.com/cdk/v2/guide/cli-telemetry.html
        ("CDK_DISABLE_CLI_TELEMETRY".to_string(), "true".to_string()),
    ])
}

#[cfg(feature = "clo")]
pub(in crate::commands::build::implementation) fn pulumi_preview_args() -> [&'static str; 2] {
    ["preview", "--skip-plugin-pre-install"]
}

#[cfg(feature = "clo")]
pub(in crate::commands::build::implementation) async fn build_cloud(root: &Path) -> Result<()> {
    let kind = mgc_cloud_adapter::detect_type(root).ok_or_else(|| {
        CloudBuildError::FrameworkNotDetected {
            root: root.to_path_buf(),
        }
    })?;
    match kind {
        mgc_cloud_adapter::CloudType::Cdk => {
            if !cloud_local_executable_exists(root, "cdk", cfg!(windows)) {
                return Err(crate::error::web_missing_executable(
                    "cdk",
                    "mgc install",
                    root,
                ));
            }
            let args = node_bin_args(root, "cdk", &["synth"])?
                .into_iter()
                .map(|a| a.to_string_lossy().to_string())
                .collect::<Vec<_>>();
            let local_bin = root.join("node_modules").join(".bin");
            // Use isolated home variables so Node has a profile without reading
            // the developer's home directory or cloud credentials.
            // (Cấp profile tạm cho Node mà không đọc home hay credential cloud của user.)
            let cdk_home = tempfile::tempdir()?;
            let env = cloud_cdk_synth_env(&local_bin, cdk_home.path())?;
            info(&format!("cdk synth: node {}", args.join(" ")));
            let opts = mgc_exec::prelude::ExecOptions {
                cwd: Some(root.to_path_buf()),
                env,
                clean_env: true,
                execution_scope: Some(mgc_exec::allowlist::ExecutionScope::BuildRunner),
                ..Default::default()
            };
            mgc_exec::prelude::run_inherited("node", &args, &opts)?;
        }
        mgc_cloud_adapter::CloudType::Pulumi => {
            if tool_unavailable("pulumi") {
                return Err(CloudBuildError::ToolchainMissing { tool: "pulumi" }.into());
            }
            run_allowlisted_tool(root, "pulumi", &pulumi_preview_args())?;
        }
        mgc_cloud_adapter::CloudType::Terraform => {
            if tool_unavailable("terraform") {
                return Err(CloudBuildError::ToolchainMissing { tool: "terraform" }.into());
            }
            run_allowlisted_tool(root, "terraform", &["plan"])?;
        }
        mgc_cloud_adapter::CloudType::Cloudflare => {
            return Err(CloudBuildError::CloudflareBuildInCicdCore.into());
        }
    }
    // IaC builds emit a plan rather than an artefact tree, so the thing worth
    // describing is the configuration that was evaluated.
    // Build IaC xuất plan chứ không phải cây artefact, nên thứ đáng mô tả là
    // cấu hình đã được đánh giá.
    crate::commands::build::post_build::run_for_core(root, "cloud", root, false)?;
    Ok(())
}
