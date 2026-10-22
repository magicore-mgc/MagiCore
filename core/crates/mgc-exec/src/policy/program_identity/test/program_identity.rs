use crate::policy::tool_basename;

#[test]
fn basename_normalizes_separators_case_and_windows_suffixes() {
    assert_eq!(tool_basename("/usr/bin/NPM"), "npm");
    assert_eq!(tool_basename(r"C:\tools\NPM.CMD"), "npm");
    assert_eq!(tool_basename("pnpm.exe"), "pnpm");
}
