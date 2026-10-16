#![allow(clippy::unwrap_used, unsafe_code)]
// Scoped registry credential regression tests with isolated project/home fixtures.
// Giữ token cùng scope, từ chối credential scope lỗi trước khi native transport dùng.
use mgc_config::{chain, npmrc, project, registry};
fn config_with(registries: Vec<registry::Registry>) -> project::ProjectConfig {
    let mut config = project::ProjectConfig::new("fixture", "web");
    config.registries = registries;
    config
}
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
struct Fixture {
    _lock: std::sync::MutexGuard<'static, ()>,
    root: tempfile::TempDir,
    saved: Vec<(&'static str, Option<std::ffi::OsString>)>,
}
impl Fixture {
    fn new() -> Self {
        let lock = ENV_LOCK.lock().unwrap();
        let root = tempfile::tempdir().unwrap();
        let keys = [
            "HOME",
            "MAGICORE_WEB_REGISTRY_URL",
            "MAGICORE_WEB_REGISTRY_TOKEN",
        ];
        let saved = keys
            .iter()
            .map(|key| (*key, std::env::var_os(key)))
            .collect();
        unsafe {
            std::env::set_var("HOME", root.path());
            std::env::remove_var(keys[1]);
            std::env::remove_var(keys[2]);
        }
        Self {
            _lock: lock,
            root,
            saved,
        }
    }
    fn npmrc(&self, text: &str) {
        std::fs::write(self.root.path().join(".npmrc"), text).unwrap();
    }
    fn checked(
        &self,
        cfg: Option<&project::ProjectConfig>,
    ) -> anyhow::Result<Vec<registry::Registry>> {
        chain::registry_chain_checked(Some(self.root.path()), cfg)
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        for (key, value) in &self.saved {
            unsafe {
                match value {
                    Some(value) => std::env::set_var(key, value),
                    None => std::env::remove_var(key),
                }
            }
        }
    }
}
#[test]
fn path_token_beats_host_token() {
    let f = Fixture::new();
    f.npmrc("registry=https://registry.example/team/\n//registry.example/:_authToken=root-fixture\n//registry.example/team/:_authToken=team-fixture\n");
    assert!(f.checked(None).unwrap()[0].token.as_deref() == Some("team-fixture"));
}
#[test]
fn env_url_keeps_matching_npmrc_auth() {
    let f = Fixture::new();
    f.npmrc("//127.0.0.1:4351/team/:_authToken=port-fixture\n");
    unsafe { std::env::set_var("MAGICORE_WEB_REGISTRY_URL", "http://127.0.0.1:4351/team/") };
    let c = f.checked(None).unwrap();
    assert_eq!(c.len(), 1);
    assert!(c[0].token.as_deref() == Some("port-fixture"));
}
#[test]
fn env_token_overrides_npmrc_auth() {
    let f = Fixture::new();
    f.npmrc("//registry.example/:_authToken=root-fixture\n");
    unsafe {
        std::env::set_var("MAGICORE_WEB_REGISTRY_URL", "https://registry.example");
        std::env::set_var("MAGICORE_WEB_REGISTRY_TOKEN", "env-fixture")
    };
    assert!(f.checked(None).unwrap()[0].token.as_deref() == Some("env-fixture"));
}
#[test]
fn config_token_overrides_npmrc_auth() {
    let f = Fixture::new();
    f.npmrc("//registry.example/:_authToken=root-fixture\n");
    let mut r = registry::Registry::new("configured".into(), "https://registry.example".into());
    r.token = Some("config-fixture".into());
    let cfg = config_with(vec![r]);
    assert!(f.checked(Some(&cfg)).unwrap()[0].token.as_deref() == Some("config-fixture"));
}
#[test]
fn explicit_token_does_not_allow_url_userinfo() {
    let f = Fixture::new();
    unsafe {
        std::env::set_var(
            "MAGICORE_WEB_REGISTRY_URL",
            "https://fixture-user:fixture-password@registry.example",
        );
        std::env::set_var("MAGICORE_WEB_REGISTRY_TOKEN", "env-fixture")
    };
    assert!(f.checked(None).is_err());
}
#[test]
fn conflicting_scopes_are_not_silently_fallback() {
    let f = Fixture::new();
    f.npmrc("registry=https://registry.example/team/\n//registry.example/team/:_authToken=a-fixture\n//REGISTRY.EXAMPLE:443/team/:_authToken=b-fixture\n");
    assert!(f.checked(None).is_err());
}
#[cfg(unix)]
#[test]
fn malformed_project_input_cannot_fall_back() {
    let f = Fixture::new();
    let other = tempfile::NamedTempFile::new().unwrap();
    std::os::unix::fs::symlink(other.path(), f.root.path().join(".npmrc")).unwrap();
    assert!(f.checked(None).is_err());
}

#[test]
fn case_sensitive_registry_paths_stay_distinct() {
    let f = Fixture::new();
    let cfg = config_with(vec![
        registry::Registry::new("upper".into(), "https://registry.example/Team".into()),
        registry::Registry::new("lower".into(), "https://registry.example/team".into()),
    ]);
    let chain = f.checked(Some(&cfg)).unwrap();
    assert_eq!(chain.len(), 3);
    assert_eq!(chain[0].url, "https://registry.example/Team");
    assert_eq!(chain[1].url, "https://registry.example/team");
}
#[test]
fn equivalent_hosts_and_default_ports_dedupe() {
    let f = Fixture::new();
    let cfg = config_with(vec![
        registry::Registry::new(
            "explicit".into(),
            "https://REGISTRY.example:443/Team/".into(),
        ),
        registry::Registry::new("implicit".into(), "https://registry.example/Team".into()),
    ]);
    assert_eq!(f.checked(Some(&cfg)).unwrap().len(), 2);
}

#[test]
fn query_in_auth_scope_is_rejected() {
    let rc = npmrc::NpmRc::parse("//registry.example/?team:_authToken=fixture\n").unwrap();
    assert!(
        rc.token_for_url_checked("https://registry.example/foreign/")
            .is_err(),
        "query scope must fail without widening credential authorization"
    );
}
#[test]
fn fragment_in_auth_scope_is_rejected() {
    let rc = npmrc::NpmRc::parse("//registry.example/#team:_authToken=fixture\n").unwrap();
    assert!(
        rc.token_for_url_checked("https://registry.example/foreign/")
            .is_err(),
        "fragment scope must fail without widening credential authorization"
    );
}

#[test]
fn credential_retains_selected_scope_not_registry_path() {
    let rc = npmrc::NpmRc::parse("//REGISTRY.example:443/team/:_authToken=fixture\n").unwrap();
    let selected = rc
        .credential_for_url_checked("https://registry.example/team/sub/package")
        .unwrap()
        .unwrap();
    assert_eq!(selected.scope.as_str(), "https://registry.example/team/");
    assert!(selected.token == "fixture");
}
#[test]
fn host_credential_retains_root_scope() {
    let rc = npmrc::NpmRc::parse("//registry.example/:_authToken=fixture\n").unwrap();
    let selected = rc
        .credential_for_url_checked("https://registry.example/team/")
        .unwrap()
        .unwrap();
    assert_eq!(selected.scope.path(), "/");
}

// Keep scoped credentials through the chain, including public fallback boundaries.
// Khóa contract scope qua chain; registry public không nhận token private.
#[test]
fn scoped_chain_preserves_npmrc_path_and_public_boundary() {
    let fixture = Fixture::new();
    fixture.npmrc(
        "registry=https://registry.example/team/\n//registry.example/team/:_authToken=fixture\n",
    );
    let entries = chain::registry_chain_scoped_checked(Some(fixture.root.path()), None).unwrap();
    assert_eq!(
        entries[0].credential_scope.as_ref().unwrap().path(),
        "/team/"
    );
    assert!(entries[0].registry.token.as_deref() == Some("fixture"));
    assert!(entries.last().unwrap().credential_scope.is_none());
    assert!(entries.last().unwrap().registry.token.is_none());
}

// Explicit config credentials keep their existing origin-wide contract.
// Token config tường minh không bị thu hẹp theo đường metadata của registry.
#[test]
fn explicit_config_credential_scope_is_origin_wide() {
    let fixture = Fixture::new();
    let mut registry =
        registry::Registry::new("configured".into(), "https://registry.example/team/".into());
    registry.token = Some("fixture".into());
    let config = config_with(vec![registry]);
    let entries =
        chain::registry_chain_scoped_checked(Some(fixture.root.path()), Some(&config)).unwrap();
    assert_eq!(
        entries[0].credential_scope.as_ref().unwrap().as_str(),
        "https://registry.example/"
    );
}
