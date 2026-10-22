//! OCI image configuration models tests.
//! Kiểm thử model cấu hình oci image

use super::*;

#[test]
fn oci_image_config_uses_schema_casing_and_reads_legacy_names() {
    let image_config = OciImageConfig {
        created: "2026-10-03T00:00:00Z".into(),
        architecture: "amd64".into(),
        os: "linux".into(),
        config: Some(OciConfig {
            user: Some("1000:1000".into()),
            exposed_ports: Some(std::collections::HashMap::from([(
                "8080/tcp".into(),
                serde_json::json!({}),
            )])),
            env: Some(vec!["MODE=prod".into()]),
            entrypoint: Some(vec!["/init".into()]),
            cmd: Some(vec!["serve".into()]),
            volumes: Some(std::collections::HashMap::from([(
                "/data".into(),
                serde_json::json!({}),
            )])),
            working_dir: Some("/app".into()),
            labels: Some(std::collections::HashMap::from([(
                "org.example.name".into(),
                "image".into(),
            )])),
        }),
        rootfs: Some(OciRootFs {
            r#type: "layers".into(),
            diff_ids: vec!["sha256:layer".into()],
        }),
        history: Some(vec![OciHistory {
            created: "2026-10-03T00:00:00Z".into(),
            created_by: Some("mgc".into()),
            empty_layer: Some(false),
            comment: None,
        }]),
        annotations: None,
    };
    let value = serde_json::to_value(&image_config).unwrap();
    assert_eq!(value["config"]["User"], "1000:1000");
    assert!(value["config"].get("user").is_none());
    for key in [
        "ExposedPorts",
        "Env",
        "Entrypoint",
        "Cmd",
        "Volumes",
        "WorkingDir",
        "Labels",
    ] {
        assert!(value["config"].get(key).is_some(), "missing {key}");
    }
    assert_eq!(value["rootfs"]["diff_ids"][0], "sha256:layer");
    assert!(value["rootfs"].get("diffIds").is_none());
    assert_eq!(value["history"][0]["created_by"], "mgc");
    assert_eq!(value["history"][0]["empty_layer"], false);

    let legacy: OciImageConfig = serde_json::from_value(serde_json::json!({
        "created": "2026-10-03T00:00:00Z",
        "architecture": "amd64",
        "os": "linux",
        "config": {
            "user": "1000:1000",
            "exposed_ports": {"8080/tcp": {}},
            "env": ["MODE=prod"],
            "entrypoint": ["/init"],
            "cmd": ["serve"],
            "volumes": {"/data": {}},
            "working_dir": "/app",
            "labels": {"org.example.name": "image"}
        },
        "rootfs": {"type": "layers", "diffIds": ["sha256:layer"]},
        "history": [{
            "created": "2026-10-03T00:00:00Z",
            "createdBy": "mgc",
            "emptyLayer": false
        }]
    }))
    .unwrap();
    assert_eq!(legacy.config.unwrap().working_dir.as_deref(), Some("/app"));
    assert_eq!(legacy.rootfs.unwrap().diff_ids, ["sha256:layer"]);
    assert_eq!(
        legacy.history.unwrap()[0].created_by.as_deref(),
        Some("mgc")
    );
}
