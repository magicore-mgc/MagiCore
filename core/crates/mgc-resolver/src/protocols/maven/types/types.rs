/// One `<dependency>` entry of a POM.
/// Một entry `<dependency>` của POM.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PomDependency {
    pub group: Option<String>,
    pub artifact: Option<String>,
    pub version: Option<String>,
    pub scope: Option<String>,
    pub optional: bool,
}
