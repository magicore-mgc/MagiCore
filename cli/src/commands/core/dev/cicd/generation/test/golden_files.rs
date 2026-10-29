//! Golden-file contract for every generated pipeline (P-7.1).
//! Contract golden file cho mọi pipeline sinh ra (P-7.1).
//!
//! A generator that quietly changes its output is a generator that breaks
//! pipelines nobody is watching. Each provider's rendered file is compared
//! byte-for-byte with a checked-in golden, so any change must be an explicit
//! edit to the golden in the same commit.
//! (Generator âm thầm đổi output là generator phá pipeline không ai để ý. File
//! của từng provider được so từng byte với golden trong repo, nên mọi thay đổi
//! phải là sửa golden tường minh trong cùng commit.)

use std::path::PathBuf;

// `#[path]` registration puts this file inside the test module, so the parent
// is `generation` itself — same rule its sibling test file follows.
// (`#[path]` đặt file này trong module test nên parent chính là `generation`,
// cùng quy tắc file test anh em.)
use super::*;
use mgc_cicd_adapter::CicdProvider;

/// Providers whose output is a single pipeline file.
/// Provider có output là một file pipeline.
const SINGLE_FILE_PROVIDERS: &[(&str, CicdProvider)] = &[
    ("github-actions", CicdProvider::GithubActions),
    ("gitlab", CicdProvider::Gitlab),
    ("circleci", CicdProvider::CircleCi),
    ("jenkins", CicdProvider::Jenkins),
    ("azure-pipelines", CicdProvider::AzurePipelines),
    ("bitbucket-pipelines", CicdProvider::BitbucketPipelines),
];

fn golden_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden")
        .join("cicd")
}

fn compare(provider: &str, rendered: &str) {
    let path = golden_dir().join(format!("{provider}.golden"));
    // Regeneration is an explicit, reviewable action: the golden is written
    // and the test still fails, so the change must be committed deliberately.
    // (Tạo lại là hành động tường minh, review được: golden được ghi và test
    // vẫn fail, nên thay đổi phải được commit có chủ đích.)
    if std::env::var_os("MGC_UPDATE_GOLDEN").is_some() {
        std::fs::create_dir_all(golden_dir()).unwrap();
        std::fs::write(&path, rendered).unwrap();
        eprintln!(
            "golden '{}' regenerated; re-run without MGC_UPDATE_GOLDEN and review the diff",
            path.display()
        );
        return;
    }
    let expected = std::fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "missing golden '{}': {error}; regenerate with MGC_UPDATE_GOLDEN=1",
            path.display()
        )
    });
    assert_eq!(
        rendered,
        expected,
        "generated {provider} pipeline differs from '{}'; if the change is intended, update the golden",
        path.display()
    );
}

/// P-7.1: all six single-file providers match their golden byte-for-byte.
/// (P-7.1: sáu provider một file khớp golden từng byte.)
#[test]
fn every_single_file_provider_matches_its_golden() {
    for (name, provider) in SINGLE_FILE_PROVIDERS {
        let plan = generation_plan(*provider, "demo-project").expect("provider has a plan");
        assert_eq!(plan.len(), 1, "{name} emits exactly one file");
        compare(name, &plan[0].1);
    }
}

/// P-7.1: the ArgoCD pair (AppProject + Application) also matches goldens.
/// (P-7.1: cặp ArgoCD (AppProject + Application) cũng khớp golden.)
#[test]
fn argocd_pair_matches_its_goldens() {
    let plan = generation_plan(CicdProvider::Argocd, "demo-project").expect("argocd plan");
    assert_eq!(plan.len(), 2, "ArgoCD emits project + application");
    compare("argocd-app-project", &plan[0].1);
    compare("argocd-application", &plan[1].1);
}

/// Every golden must be reproducible from the templates alone: rendering twice
/// yields identical text (no timestamps, no random ids).
/// (Mọi golden phải tái lập được từ template: render hai lần cho cùng text,
/// không timestamp, không id ngẫu nhiên.)
#[test]
fn rendering_is_deterministic() {
    for (name, provider) in SINGLE_FILE_PROVIDERS {
        let first = generation_plan(*provider, "demo-project").unwrap();
        let second = generation_plan(*provider, "demo-project").unwrap();
        assert_eq!(
            first[0].1, second[0].1,
            "{name} must render deterministically"
        );
    }
    // The GitHub workflow is the only template rendered with a literal label,
    // so render it twice as well.
    // (Workflow GitHub là template duy nhất render với nhãn literal, nên
    // render hai lần nữa.)
    let a = render_ci_template(
        super::super::super::templates::GITHUB_WORKFLOW_TEMPLATE,
        "CI",
    );
    let b = render_ci_template(
        super::super::super::templates::GITHUB_WORKFLOW_TEMPLATE,
        "CI",
    );
    assert_eq!(a, b);
}
