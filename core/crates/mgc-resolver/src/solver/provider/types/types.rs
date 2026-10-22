/// One dependency of a resolved package: name + version spec + flags.
use mgc_types::PackageName;
#[derive(Debug, Clone)]
pub struct ResolvedDep {
    pub package: PackageName,
    pub spec: String,
    pub optional: bool,
    pub peer: bool,
}
