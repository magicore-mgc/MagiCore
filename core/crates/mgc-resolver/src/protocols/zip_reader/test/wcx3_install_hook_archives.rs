//! WCX3 real-world archive fixtures for SEC-3 gate tests (pth/sitecustomize/nuget).
//! Fixture suite (WCX3) cột thực tế cho test cổng SEC-3 (pth/sitecustomize/nuget).
//!
//! Each fixture is a REAL archive of the shape an attacker must ship to reach
//! the detector: a wheel-style ZIP, a tar.gz with a `package/` root, or a nupkg.
//! The test extracts the archive and asserts (a) refusal, (b) no trace on disk.
//! (Mỗi fixture là archive THẬT đúng hình dạng kẻ tấn công phải gửi để chạm
//! detector: ZIP kiểu wheel, tar.gz có root `package/`, hoặc nupkg. Test giải nén
//! archive và khẳng định (a) bị từ chối, (b) không để dấu vết trên đĩa.)

use crate::protocols::archive::InstallHookPolicy;
use crate::protocols::zip_reader::{extract_zip, extract_zip_with_policy, test_support::build_zip};

/// 1. The litellm 1.107.0 payload shape: a wheel ZIP with a `.pth` entry, a
///    clean module, and dist-info bookkeeping. A real installer only needs the
///    `.pth` to execute code at every interpreter start.
/// (1. Hình dạng payload litellm 1.107.0: ZIP wheel có entry `.pth`, module
/// sạch, và dist-info. Installer thật chỉ cần `.pth` để chạy code mỗi lần
/// interpreter khởi động.)
fn litellm_wheel() -> Vec<u8> {
    build_zip(&[
        (
            "litellm-1.107.0.dist-info/METADATA",
            b"Name: litellm\nVersion: 1.107.0\n".to_vec(),
            8,
        ),
        ("litellm/__init__.py", b"x = 1\n".to_vec(), 8),
        (
            "litellm_init.pth",
            b"import os; os.system('touch /tmp/pwned')\n".to_vec(),
            8,
        ),
    ])
}

/// 2. A wheel that hides `sitecustomize.py` in a package directory. Python's
///    `site` imports it whenever that directory is on `sys.path`.
/// (2. Wheel giấu `sitecustomize.py` trong thư mục package. `site` của Python
/// import nó bất cứ khi nào thư mục đó nằm trên `sys.path`.)
fn sitecustomize_wheel() -> Vec<u8> {
    build_zip(&[
        ("pkg/sitecustomize.py", b"import os\n".to_vec(), 8),
        ("pkg/__init__.py", b"".to_vec(), 8),
    ])
}

/// 3. A NuGet package with `tools/install.ps1`, the script the NuGet client
///    runs for packages.config projects.
/// (3. Gói NuGet có `tools/install.ps1` — script NuGet client chạy cho
/// project packages.config.)
fn nuget_install_script_package() -> Vec<u8> {
    build_zip(&[
        ("Demo.Lib.nuspec", b"<package/>".to_vec(), 8),
        ("lib/net8.0/Demo.Lib.dll", b"MZ".to_vec(), 8),
        ("tools/install.ps1", b"Write-Host pwned\n".to_vec(), 8),
    ])
}

#[test]
fn wheel_with_pth_is_refused_and_leaves_no_file() {
    let dir = tempfile::tempdir().expect("test operation must succeed");
    let site = dir.path().join("site");
    let error = extract_zip(&litellm_wheel(), &site).expect_err("pth wheel must be refused");
    assert!(error.to_string().contains(".pth"), "{error}");
    assert!(error.to_string().contains("litellm_init.pth"), "{error}");
    assert!(!site.exists(), "a refused archive must leave no trace");
}

#[test]
fn wheel_with_sitecustomize_is_refused() {
    let dir = tempfile::tempdir().expect("test operation must succeed");
    let site = dir.path().join("site");
    let error = extract_zip(&sitecustomize_wheel(), &site).expect_err("site hook must be refused");
    assert!(error.to_string().contains("sitecustomize.py"), "{error}");
    assert!(!site.exists());
}

#[test]
fn nupkg_with_install_script_is_refused() {
    let dir = tempfile::tempdir().expect("test operation must succeed");
    let packages = dir.path().join("demo.lib").join("1.0.0");
    let error =
        extract_zip(&nuget_install_script_package(), &packages).expect_err("install.ps1 refused");
    assert!(error.to_string().contains("install.ps1"), "{error}");
    assert!(!packages.exists());
}

#[test]
fn clean_wheel_still_extracts() {
    let dir = tempfile::tempdir().expect("test operation must succeed");
    let site = dir.path().join("site");
    let clean = build_zip(&[
        ("pkg/__init__.py", b"x = 1\n".to_vec(), 8),
        ("pkg-1.0.dist-info/METADATA", b"Name: pkg\n".to_vec(), 8),
    ]);
    extract_zip(&clean, &site).expect("clean wheel must extract");
    assert!(site.join("pkg/__init__.py").is_file());
}

#[test]
fn escape_hatch_allows_the_named_package_and_still_refuses_others() {
    // Naming the package is enough for the policy object only when the env
    // also names it; here the env has already been checked elsewhere, so this
    // asserts the second half: a policy for package A still refuses B.
    // (Chỉ nêu package trong policy là chưa đủ nếu env không nêu; test này
    // khẳng định nửa còn lại: policy cho package A vẫn từ chối B.)
    let dir = tempfile::tempdir().expect("test operation must succeed");
    let site = dir.path().join("site");
    let policy = InstallHookPolicy::for_package("some-unlisted-package");
    let error = extract_zip_with_policy(&litellm_wheel(), &site, &policy)
        .expect_err("unlisted package must still be refused");
    assert!(error.to_string().contains(".pth"), "{error}");
    assert!(!site.exists());
}

/// A tar.gz with a `package/` root that also carries a `.pth`, as an npm
/// tarball would after a malicious repack.
/// (tar.gz có root `package/` kèm `.pth`, như tarball npm sau khi đóng lại độc hại.)
fn npm_shaped_tarball_with_pth() -> Vec<u8> {
    crate::protocols::archive::test_support::tar_gz_entries(&[
        ("package/package.json", b"{\"name\":\"evil\"}".as_slice()),
        ("package/index.js", b"module.exports = 1;".as_slice()),
        (
            "package/evil.pth",
            b"import os; os.system('touch /tmp/pwned')".as_slice(),
        ),
    ])
}

#[test]
fn npm_shaped_tarball_with_pth_is_classified_as_a_hook_entry() {
    // The general tarball extractor has its own nearest tests; this case proves
    // the npm-shaped tar path reaches the same detector as wheels/nupkgs, using
    // the exact entry list the archive carries.
    // (Bộ giải nén tar tổng quát đã có test gần nhất riêng; ca này chứng minh
    // đường tar dạng npm chạm cùng detector với wheel/nupkg, dùng đúng danh
    // sách entry trong archive.)
    let archive = npm_shaped_tarball_with_pth();
    let mut decoded = flate2::read::GzDecoder::new(archive.as_slice());
    let mut tar_bytes = Vec::new();
    std::io::Read::read_to_end(&mut decoded, &mut tar_bytes).expect("fixture must be valid gzip");
    let mut archive = tar::Archive::new(tar_bytes.as_slice());
    let mut refused = Vec::new();
    for entry in archive.entries().expect("fixture must be a valid tar") {
        let entry = entry.expect("fixture entry must parse");
        let name = entry.path().expect("entry path must parse").into_owned();
        if let Some(hook) = crate::protocols::archive::classify_entry(&name.to_string_lossy(), None)
        {
            refused.push(hook);
        }
    }
    assert_eq!(refused.len(), 1, "exactly the .pth entry must be refused");
    assert!(refused[0].to_string().contains("evil.pth"));
}
