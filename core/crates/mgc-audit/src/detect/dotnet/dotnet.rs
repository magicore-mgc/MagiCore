//! Detect the existing .NET manifest lane.
//! Nhận diện lane manifest .NET hiện có.
use crate::contract::ScanStep;
use std::path::Path;

/// *.csproj / *.sln → OSV NuGet (dotnet). The lane reads root and
/// per-solution-project packages.lock.json pins; without any the
/// scanner reports honest Unsupported with remediation.
/// (*.csproj/*.sln → OSV NuGet.)
pub fn dotnet_step(project_root: &Path) -> Option<ScanStep<'static>> {
    let has_csproj = std::fs::read_dir(project_root)
        .map(|it| {
            it.filter_map(|e| e.ok()).any(|e| {
                e.file_name()
                    .to_str()
                    .is_some_and(|n| n.ends_with(".csproj") || n.ends_with(".sln"))
            })
        })
        .unwrap_or(false);
    if !has_csproj {
        return None;
    }
    let root = project_root.to_path_buf();
    Some(ScanStep {
        ecosystem: "dotnet",
        scanner: "osv-nuget",
        run: Box::new(move || {
            let root = root.clone();
            Box::pin(async move { crate::scanners::audit_dotnet(&root).await })
        }),
    })
}
