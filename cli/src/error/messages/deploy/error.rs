//! Deployment, container, and Kubernetes diagnostics.
//! Lỗi deploy, container và Kubernetes.

use anyhow::Error;
use anyhow::anyhow;

/// deploy target missing `stack` in [deploy] targets
pub fn deploy_target_missing_stack() -> Error {
    anyhow!("aws target is missing `stack` in [deploy] targets — example: stack = \"my-infra\"")
}

/// deploy target adapter unknown
pub fn deploy_target_unknown(other: &str) -> Error {
    anyhow!("deploy target '{other}' has no adapter — supported: aws | cloudflare | gcp")
}

/// no deploy targets configured
pub fn no_deploy_targets() -> Error {
    anyhow!("no deploy target — add [deploy] targets to mgc.toml")
}

pub fn deploy_not_implemented(cloud: &str) -> Error {
    anyhow!(
        "'mgc deploy' for '{cloud}' has no MagiCore-native provider engine yet; no external provider CLI was started"
    )
}

/// Container scaffolding cannot map a split web project to one runtime image.
/// Không thể gộp fullstack/monorepo nhiều runtime vào một image container.
/// Container scaffolding currently has a verified Node web runtime only.
/// Lane container hiện chỉ xác minh runtime web Node.
pub fn web_container_node_lane_required() -> Error {
    anyhow!(
        "container scaffolding currently supports Node web projects only; non-Node web lanes are not implemented"
    )
}

/// A requested container action has no implemented MGC lifecycle profile.
/// Lệnh container chỉ được tạo khi có profile lifecycle MGC đã triển khai.
/// Kubernetes output is limited to the verified HTTP service lane.
/// Kubernetes chỉ được sinh cho lane dịch vụ HTTP đã xác minh.
pub fn k8s_profile_unsupported(core: &str) -> Error {
    anyhow!(
        "Kubernetes scaffolding is currently supported only for Web HTTP services; core '{core}' has no verified workload profile"
    )
}
