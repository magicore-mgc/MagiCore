//! Verify the stable public paths for publish data contracts.
//! Kiểm tra public path ổn định của các contract dữ liệu publish.

use mgc_types::publish::{PublishOptions as ModuleOptions, PublishSummary as ModuleSummary};
use mgc_types::{PublishOptions, PublishSummary};

#[test]
fn crate_root_and_module_paths_export_the_same_types() {
    let options: ModuleOptions = PublishOptions::default();
    let summary: ModuleSummary = PublishSummary {
        name: "demo".into(),
        version: "1.0.0".into(),
        tag: "latest".into(),
        size: 10,
        unpacked_size: 20,
        shasum: "sha1-demo".into(),
        integrity: "sha512-demo".into(),
        files: 1,
        entry_count: 1,
    };

    assert_eq!(options.tag, "latest");
    assert_eq!(summary.entry_count, 1);
}
