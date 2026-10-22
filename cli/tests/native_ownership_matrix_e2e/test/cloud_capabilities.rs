//! Cloud Capabilities lifecycle E2E cases. / Test E2E lifecycle cho cloud capabilities.

use crate::native_ownership_matrix_e2e::support::{MatrixSandbox, write};
use tempfile::TempDir;

pub(crate) fn matrix_cloud_dependency_commands_fail_closed_without_native_manifest() {
    for (framework, commands) in [
        (
            "cdk",
            vec![
                vec!["install-clo"],
                vec!["add-clo", "constructs"],
                vec!["remove-clo", "constructs"],
                vec!["update-clo", "constructs"],
            ],
        ),
        (
            "terraform",
            vec![
                vec!["install-clo"],
                vec!["add-clo", "random"],
                vec!["remove-clo", "random"],
                vec!["update-clo", "random"],
            ],
        ),
    ] {
        for command in commands {
            let project = TempDir::new().unwrap();
            write(
                project.path(),
                "mgc.toml",
                &format!("name = \"m\"\necosystem = \"cloud\"\n[cloud]\ntype = \"{framework}\"\n"),
            );
            if framework == "terraform" {
                write(project.path(), "main.tf", "terraform {}\n");
            }
            // A CDK/Pulumi project without package.json is not an admitted
            // native JS lane; Terraform is never a package dependency lane.
            let sandbox = MatrixSandbox::new();
            let (code, out) = sandbox.run(&command, project.path());
            assert_ne!(
                code,
                Some(0),
                "clo/{framework} {} without package.json must refuse:\n{out}",
                command.join(" ")
            );
            assert!(
                out.contains("unsupported")
                    || out.contains("not supported")
                    || out.contains("native"),
                "refusal should explain missing native ownership:\n{out}"
            );
            sandbox.assert_no_spawn(&command.join(" "));
        }
    }
}

pub(crate) fn cloud_dev_and_deploy_fail_closed_without_provider_cli_spawn() {
    for framework in ["terraform", "cdk", "pulumi", "cloudflare"] {
        let project = TempDir::new().unwrap();
        write(
            project.path(),
            "mgc.toml",
            &format!(
                "name = \"cloud-e2e\"\necosystem = \"cloud\"\n[cloud]\ntype = \"{framework}\"\n"
            ),
        );
        if framework == "terraform" {
            write(project.path(), "main.tf", "terraform {}\n");
        }
        let sandbox = MatrixSandbox::new();
        for args in [
            vec!["--core", "clo", "dev"],
            vec!["deploy"],
            vec!["deploy", "--run"],
        ] {
            let (code, out) = sandbox.run(&args, project.path());
            assert_ne!(
                code,
                Some(0),
                "cloud/{framework} {} must not imply a native dev/deploy succeeded:\n{out}",
                args.join(" ")
            );
            assert!(
                out.contains("MagiCore-native provider engine"),
                "refusal must identify missing native ownership:\n{out}"
            );
            sandbox.assert_no_spawn(&format!("cloud/{framework} {}", args.join(" ")));
        }
    }
}
