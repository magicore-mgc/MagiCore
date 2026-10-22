use anyhow::Result;
use mgc_config::project::ProjectConfig;
use mgc_pack::tarball::PackResult;
use mgc_types::PublishSummary;

use crate::commands::publish::PublishArgs;

pub(in crate::commands::publish) fn print_publish_summary(
    args: &PublishArgs,
    project: &ProjectConfig,
    new_version: String,
    pack_result: &PackResult,
    file_count: usize,
) -> Result<()> {
    let summary = PublishSummary {
        name: project.name.clone(),
        version: new_version,
        tag: args.tag.clone().unwrap_or_else(|| "latest".into()),
        size: pack_result.size,
        unpacked_size: pack_result.unpacked_size,
        shasum: pack_result.shasum.clone(),
        integrity: pack_result.integrity.clone(),
        files: file_count,
        entry_count: pack_result.entry_count,
    };

    if args.json {
        println!("{}", serde_json::to_string_pretty(&summary)?);
    } else {
        println!("✓ Published {}@{}", summary.name, summary.version);
        println!("  tag: {}", summary.tag);
        println!(
            "  size: {} bytes (unpacked: {})",
            summary.size, summary.unpacked_size
        );
        println!("  shasum: {}", summary.shasum);
        println!("  integrity: {}", summary.integrity);
        println!(
            "  files: {}, entries: {}",
            summary.files, summary.entry_count
        );
    }

    Ok(())
}
