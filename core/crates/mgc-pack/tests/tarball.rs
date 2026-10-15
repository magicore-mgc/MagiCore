#![allow(clippy::unwrap_used)]
//! Integration tests for tarball builder — test riêng tại test/ (RULE §5)
use mgc_pack::tarball::pack;
use std::fs;
use std::io::Write;

fn write(root: &std::path::Path, rel: &str, content: &str) {
    let p = root.join(rel);
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    let mut f = fs::File::create(p).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}

#[test]
fn pack_produces_deterministic_tarball() {
    let src_tmp = tempfile::tempdir().unwrap();
    let root = src_tmp.path();
    write(root, "package.json", r#"{"name":"x","version":"1.0.0"}"#);
    write(root, "src/index.js", "export {}");

    // output OUTSIDE root — tránh self-pack (file tgz bị quét lại)
    let out_tmp = tempfile::tempdir().unwrap();
    let out1 = out_tmp.path().join("a.tgz");
    let out2 = out_tmp.path().join("b.tgz");
    let r1 = pack(root, &out1, "x-1.0.0").unwrap();
    let r2 = pack(root, &out2, "x-1.0.0").unwrap();

    let d1 = fs::read(&out1).unwrap();
    let d2 = fs::read(&out2).unwrap();
    assert_eq!(d1, d2, "same input must produce byte-identical tarballs");

    assert_eq!(r1.shasum, r2.shasum);
    assert_eq!(r1.integrity, r2.integrity);
    assert!(r1.shasum.len() == 40, "sha1 hex must be 40 characters");
    assert!(r1.integrity.starts_with("sha512-"));
    assert!(r1.size > 0);
    assert_eq!(r1.unpacked_size, 39); // package.json (30) + src/index.js (9) = 39
    assert_eq!(r1.entry_count, 2);
}

#[test]
fn pack_uses_prefix_in_entries() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write(root, "package.json", "{}");

    let out = tmp.path().join("p.tgz");
    pack(root, &out, "x-1.0.0").unwrap();
    let data = fs::read(&out).unwrap();
    let tarball = fs::File::open(&out).unwrap();
    let mut gz = flate2::read::GzDecoder::new(tarball);
    let mut archive = tar::Archive::new(&mut gz);
    let names: Vec<String> = archive
        .entries()
        .unwrap()
        .map(|e| e.unwrap().path().unwrap().to_string_lossy().to_string())
        .collect();
    assert!(names.contains(&"x-1.0.0/package.json".to_string()));
    assert!(!data.is_empty());
}

#[test]
fn repacking_the_same_output_inside_package_does_not_pack_itself() {
    let root = tempfile::tempdir().unwrap();
    write(root.path(), "package.json", "{}");
    let output = root.path().join("package.tgz");
    let first = pack(root.path(), &output, "package").unwrap();
    let bytes = fs::read(&output).unwrap();
    let second = pack(root.path(), &output, "package").unwrap();
    assert_eq!(first.entry_count, 1);
    assert_eq!(second.entry_count, 1);
    assert_eq!(fs::read(&output).unwrap(), bytes);
    assert_eq!(first.integrity, second.integrity);
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 2);
}

#[test]
fn failed_pack_keeps_previous_output_and_removes_staging_file() {
    let root = tempfile::tempdir().unwrap();
    let output_dir = tempfile::tempdir().unwrap();
    let output = output_dir.path().join("package.tgz");
    write(root.path(), "package.json", "{}");
    fs::write(&output, "previous archive").unwrap();
    assert!(pack(root.path(), &output, "../escape").is_err());
    assert_eq!(fs::read(&output).unwrap(), b"previous archive");
    assert_eq!(fs::read_dir(output_dir.path()).unwrap().count(), 1);
}

#[cfg(unix)]
#[test]
fn forced_readme_inclusion_cannot_follow_a_link_outside_package() {
    let root = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let output = outside.path().join("package.tgz");
    let secret = outside.path().join("secret");
    fs::write(&secret, "private content").unwrap();
    std::os::unix::fs::symlink(&secret, root.path().join("README.md")).unwrap();
    assert!(pack(root.path(), &output, "package").is_err());
    assert!(!output.exists());
    assert_eq!(fs::read(&secret).unwrap(), b"private content");
    assert_eq!(fs::read_dir(outside.path()).unwrap().count(), 1);
}

#[cfg(unix)]
#[test]
fn executable_files_keep_execution_bits_without_special_permission_bits() {
    use std::os::unix::fs::PermissionsExt;
    let root = tempfile::tempdir().unwrap();
    let output_dir = tempfile::tempdir().unwrap();
    write(root.path(), "bin/tool", "#!/bin/sh\nexit 0\n");
    fs::set_permissions(
        root.path().join("bin/tool"),
        fs::Permissions::from_mode(0o4755),
    )
    .unwrap();
    let output = output_dir.path().join("package.tgz");
    pack(root.path(), &output, "package").unwrap();
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(
        fs::File::open(output).unwrap(),
    ));
    let entry = archive.entries().unwrap().next().unwrap().unwrap();
    assert_eq!(entry.header().mode().unwrap(), 0o755);
}

#[test]
fn large_sparse_file_round_trips_with_streamed_hashes_and_correct_sizes() {
    use base64::Engine;
    use sha2::{Digest, Sha512};
    let root = tempfile::tempdir().unwrap();
    let output_dir = tempfile::tempdir().unwrap();
    let source_size = 64 * 1024 * 1024;
    fs::File::create(root.path().join("large.bin"))
        .unwrap()
        .set_len(source_size)
        .unwrap();
    let output = output_dir.path().join("package.tgz");
    let result = pack(root.path(), &output, "package").unwrap();
    assert_eq!(result.unpacked_size, source_size);
    assert_eq!(result.size, fs::metadata(&output).unwrap().len());
    let bytes = fs::read(&output).unwrap();
    assert_eq!(
        result.integrity,
        format!(
            "sha512-{}",
            base64::engine::general_purpose::STANDARD.encode(Sha512::digest(&bytes))
        )
    );
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(
        fs::File::open(output).unwrap(),
    ));
    let mut entry = archive.entries().unwrap().next().unwrap().unwrap();
    assert_eq!(
        std::io::copy(&mut entry, &mut std::io::sink()).unwrap(),
        source_size
    );
}
