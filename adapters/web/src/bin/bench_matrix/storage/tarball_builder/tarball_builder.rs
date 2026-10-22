// Benchmark tarball artifact generation
// Tạo nội dung gzip tarball và checksum cho package fixture.
use crate::bench_matrix::*;
use base64::Engine as _;

pub(in crate::bench_matrix) fn build_tarball_bytes(
    pkg: &PackageId,
    files_per_package: usize,
) -> anyhow::Result<Vec<u8>> {
    let temp = tempfile::NamedTempFile::new()?;
    let file = temp.reopen()?;
    let encoder = GzEncoder::new(file, Compression::default());
    let mut builder = Builder::new(encoder);

    let package_json = format!(
        "{{\"name\":\"{}\",\"version\":\"{}\"}}",
        pkg.name_str(),
        pkg.version()
    );
    write_tar_entry(
        &mut builder,
        "package/package.json",
        package_json.as_bytes(),
    )?;
    write_tar_entry(
        &mut builder,
        "package/src/index.js",
        format!("export const name = '{}';\n", pkg.name_str()).as_bytes(),
    )?;
    for idx in 0..files_per_package {
        write_tar_entry(
            &mut builder,
            &format!("package/dist/chunk-{idx}.js"),
            format!("export const v{idx} = {idx};\n").as_bytes(),
        )?;
    }

    builder.finish()?;
    let encoder = builder.into_inner()?;
    encoder.finish()?;
    Ok(std::fs::read(temp.path())?)
}

pub(in crate::bench_matrix) fn sri_sha512(bytes: &[u8]) -> String {
    let digest = Sha512::digest(bytes);
    format!("sha512-{}", BASE64_STANDARD.encode(digest))
}

pub(in crate::bench_matrix) fn write_tar_entry(
    builder: &mut Builder<GzEncoder<std::fs::File>>,
    path: &str,
    data: &[u8],
) -> anyhow::Result<()> {
    let mut header = Header::new_gnu();
    header.set_size(data.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    builder.append_data(&mut header, path, data)?;
    Ok(())
}
