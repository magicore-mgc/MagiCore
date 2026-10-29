// dispatch owner — dispatch owner implementation.
// Responsibility boundary — ranh giới trách nhiệm của owner.
use super::super::cas_pull::cas_pull;
use super::super::local_models::{list_local, remove_local};
use super::super::quantize::quantize;
use super::super::registry_catalog::list;
use super::super::registry_pull::pull;
use super::super::registry_push::push;
use super::super::types::ModelOptionsError;
use super::super::types::{ModelArgs, ModelCmd};
use anyhow::Result;
pub async fn run(args: ModelArgs) -> Result<()> {
    match args.cmd {
        ModelCmd::Push {
            paths,
            repo,
            tag,
            registry,
            token,
        } => push(paths, &repo, &tag, &registry, token).await,
        ModelCmd::Pull {
            repo,
            tag,
            registry,
            output,
            revision,
            sha256,
            max_bytes,
            token,
        } => {
            if repo.starts_with("hf://") || repo.starts_with("oci://") {
                cas_pull(&repo, revision.as_deref(), sha256.as_deref(), max_bytes).await
            } else {
                if revision.is_some() || sha256.is_some() || max_bytes.is_some() {
                    return Err(ModelOptionsError::PullFlagsRequireHf.into());
                }
                pull(&repo, &tag, &registry, &output, token).await
            }
        }
        ModelCmd::List {
            local,
            registry,
            token,
        } => {
            if local {
                list_local()
            } else {
                list(&registry, token).await
            }
        }
        ModelCmd::Rm { name } => remove_local(&name),
        ModelCmd::Bench {
            model,
            path,
            samples,
            json,
        } => {
            super::super::bench::run(&model, std::path::Path::new(&path), samples, json).map(|_| ())
        }
        ModelCmd::Quantize {
            path,
            target,
            output,
        } => quantize(&path, &target, output.as_deref()),
    }
}
