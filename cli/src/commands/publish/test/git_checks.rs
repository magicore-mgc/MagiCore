//! Publish Git guard tests.
//! Kiểm thử rào chắn Git khi publish.
use super::*;

#[test]
fn publish_git_queries_are_a_closed_read_only_set() {
    assert_eq!(
        GitReadQuery::ValidateBranch("release".to_string()).argv(),
        ["check-ref-format", "refs/heads/release"]
    );
    assert_eq!(
        GitReadQuery::WorkingTreeStatus.argv(),
        ["status", "--porcelain"]
    );
    assert_eq!(
        GitReadQuery::CurrentBranch.argv(),
        [
            "rev-parse",
            "--verify",
            "--abbrev-ref",
            "--end-of-options",
            "HEAD"
        ]
    );
    assert_eq!(
        GitReadQuery::UpstreamRef("release".to_string()).argv(),
        [
            "rev-parse",
            "--verify",
            "--abbrev-ref",
            "--symbolic-full-name",
            "--end-of-options",
            "release@{upstream}"
        ]
    );
    assert_eq!(
        GitReadQuery::CommitsBehind("release".to_string()).argv(),
        [
            "rev-list",
            "--count",
            "--end-of-options",
            "release..release@{upstream}"
        ]
    );
}

#[test]
fn publish_git_executor_is_locked_to_the_selected_project_root() {
    let project_root = Path::new("/tmp/magicore-publish-project");
    let options = git_exec_options(project_root);
    assert_eq!(options.cwd.as_deref(), Some(project_root));
    assert!(options.clean_env);
}

#[test]
fn publish_git_lag_parser_fails_closed_on_invalid_output() {
    assert_eq!(parse_git_behind_count("2\n").unwrap(), 2);
    assert!(parse_git_behind_count("not-a-count").is_err());
}

#[test]
fn publish_git_checks_use_selected_repo_and_reject_a_lagging_branch() {
    let temp = tempfile::tempdir().expect("temporary git test directory");
    let project_root = temp.path().join("project");
    let remote = temp.path().join("origin.git");
    let advance = temp.path().join("advance");
    let global_config = temp.path().join("gitconfig");
    fs::create_dir_all(&project_root).expect("create project");
    fs::write(&global_config, "").expect("create isolated git config");

    let run_git = |cwd: &Path, args: &[&str]| {
        let output = Command::new("git")
            .args(args)
            .current_dir(cwd)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", &global_config)
            .output()
            .expect("run local git fixture command");
        assert!(
            output.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    };

    run_git(&project_root, &["init", "--quiet"]);
    run_git(&project_root, &["config", "user.name", "MagiCore Test"]);
    run_git(
        &project_root,
        &["config", "user.email", "test@example.invalid"],
    );
    fs::write(project_root.join("tracked.txt"), "initial\n").expect("write tracked file");
    run_git(&project_root, &["add", "tracked.txt"]);
    run_git(&project_root, &["commit", "--quiet", "-m", "initial"]);
    run_git(&project_root, &["branch", "-M", "publish-check"]);

    let remote_arg = remote.to_string_lossy().into_owned();
    run_git(
        &project_root,
        &["clone", "--quiet", "--bare", ".", &remote_arg],
    );
    run_git(&project_root, &["remote", "add", "origin", &remote_arg]);
    run_git(&project_root, &["fetch", "--quiet", "origin"]);
    run_git(
        &project_root,
        &[
            "branch",
            "--set-upstream-to=origin/publish-check",
            "publish-check",
        ],
    );
    crate::commands::publish::git_checks(Some("publish-check"), &project_root)
        .expect("selected project's branch should initially match upstream");
    assert!(crate::commands::publish::git_checks(Some("bad/branch?"), &project_root).is_err());

    let advance_arg = advance.to_string_lossy().into_owned();
    run_git(
        temp.path(),
        &["clone", "--quiet", &remote_arg, &advance_arg],
    );
    run_git(&advance, &["config", "user.name", "MagiCore Test"]);
    run_git(&advance, &["config", "user.email", "test@example.invalid"]);
    fs::write(advance.join("tracked.txt"), "remote update\n").expect("update remote file");
    run_git(&advance, &["add", "tracked.txt"]);
    run_git(&advance, &["commit", "--quiet", "-m", "remote update"]);
    run_git(&advance, &["push", "--quiet", "origin", "publish-check"]);
    run_git(&project_root, &["fetch", "--quiet", "origin"]);

    let error = crate::commands::publish::git_checks(Some("publish-check"), &project_root)
        .expect_err("publish must reject a selected branch behind its upstream");
    assert!(error.to_string().contains("lags 1 commits behind upstream"));
}
