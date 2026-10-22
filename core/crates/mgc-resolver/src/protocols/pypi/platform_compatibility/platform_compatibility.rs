/// OS/arch platform check (no Python involved).
/// (Kiểm tra OS/arch.)
pub(in crate::protocols::pypi) fn platform_matches_host(plat: &str) -> bool {
    platform_tag_matches_target(
        plat,
        std::env::consts::OS,
        std::env::consts::ARCH,
        host_platform_version(),
    )
}

pub(in crate::protocols::pypi) fn platform_tag_matches_target(
    plat: &str,
    os: &str,
    arch: &str,
    platform_version: Option<(u64, u64)>,
) -> bool {
    plat.split('.').any(|tag| match (os, arch) {
        ("macos", "aarch64") => {
            tag.starts_with("macosx_")
                && (tag.ends_with("_arm64") || tag.ends_with("_universal2"))
                && platform_floor(tag, "macosx_")
                    .zip(platform_version)
                    .is_some_and(|(required, host)| host >= required)
        }
        ("macos", "x86_64") => {
            tag.starts_with("macosx_")
                && (tag.ends_with("_x86_64") || tag.ends_with("_universal2"))
                && platform_floor(tag, "macosx_")
                    .zip(platform_version)
                    .is_some_and(|(required, host)| host >= required)
        }
        ("linux", "aarch64") => {
            tag.ends_with("_aarch64")
                && linux_wheel_floor(tag)
                    .zip(platform_version)
                    .is_some_and(|(required, host)| host >= required)
        }
        ("linux", "x86_64") => {
            tag.ends_with("_x86_64")
                && linux_wheel_floor(tag)
                    .zip(platform_version)
                    .is_some_and(|(required, host)| host >= required)
        }
        ("windows", "x86_64") => tag == "win_amd64",
        ("windows", "aarch64") => tag == "win_arm64",
        ("windows", "x86") => tag == "win32",
        _ => false,
    })
}

pub(in crate::protocols::pypi) fn platform_floor(tag: &str, prefix: &str) -> Option<(u64, u64)> {
    let mut parts = tag.strip_prefix(prefix)?.split('_');
    Some((parts.next()?.parse().ok()?, parts.next()?.parse().ok()?))
}

pub(in crate::protocols::pypi) fn linux_wheel_floor(tag: &str) -> Option<(u64, u64)> {
    if let Some(floor) = platform_floor(tag, "manylinux_") {
        return Some(floor);
    }
    if let Some(floor) = platform_floor(tag, "musllinux_") {
        return Some(floor);
    }
    match tag {
        value if value.starts_with("manylinux2014_") => Some((2, 17)),
        value if value.starts_with("manylinux2010_") => Some((2, 12)),
        value if value.starts_with("manylinux1_") => Some((2, 5)),
        _ => None,
    }
}

// SAFETY (macOS branch): sysctlbyname reads only the NUL-terminated key
// CString (alive for the sync call) and writes at most 64 bytes into the
// stack buffer whose capacity is passed in `size`; newp is null with
// newlen 0 (query only, no write); on success the kernel NUL-terminates the
// version string and `size` (checked 1..=64) bounds the CStr read — no
// memory is reinterpreted beyond that validated byte string.
// (AN TOÀN — nhánh macOS: sysctlbyname chỉ đọc key CString kết thúc NUL còn
// sống qua lời gọi và ghi tối đa 64 byte vào buffer stack đã truyền capacity
// qua `size`; newp null/newlen 0 nên chỉ đọc không ghi; kernel kết thúc NUL
// chuỗi version và `size` đã kiểm tra 1..=64 giới hạn lần đọc CStr — không
// diễn giải lại bộ nhớ ngoài chuỗi byte đã validate.)
// SAFETY (glibc branch): gnu_get_libc_version returns a pointer to glibc's
// own static version string (never freed, always NUL-terminated); the null
// check guards exotic toolchains and CStr::from_ptr reads only up to that
// terminator — no memory is reinterpreted.
// (AN TOÀN — nhánh glibc: gnu_get_libc_version trả về con trỏ tới chuỗi
// version tĩnh của glibc (không bao giờ free, luôn kết thúc NUL); kiểm tra
// null phòng toolchain lạ và CStr::from_ptr chỉ đọc tới NUL — không diễn
// giải lại bộ nhớ.)
#[allow(unsafe_code)]
pub(in crate::protocols::pypi) fn host_platform_version() -> Option<(u64, u64)> {
    #[cfg(target_os = "macos")]
    {
        use std::ffi::{CStr, CString};
        let key = CString::new("kern.osproductversion").ok()?;
        let mut buffer = [0u8; 64];
        let mut size = buffer.len();
        // Read the kernel's product version without launching a shell command.
        // (Đọc phiên bản hệ điều hành từ kernel, không chạy shell command.)
        // SAFETY: key CString + 64-byte stack buffer both alive for the sync
        // query-only call; `size` carries the capacity in. (AN TOÀN: key CString
        // và buffer 64 byte còn sống qua lời gọi truy vấn đồng bộ; `size` mang
        // capacity vào.)
        let status = unsafe {
            libc::sysctlbyname(
                key.as_ptr(),
                buffer.as_mut_ptr().cast(),
                &mut size,
                std::ptr::null_mut(),
                0,
            )
        };
        if status != 0 || size == 0 || size > buffer.len() {
            return None;
        }
        // SAFETY: status == 0 plus 1..=64 `size` check above guarantee the kernel
        // wrote a NUL-terminated string inside `buffer`. (AN TOÀN: status == 0
        // và `size` 1..=64 đã kiểm tra đảm bảo kernel đã ghi chuỗi kết thúc NUL
        // trong `buffer`.)
        let text = unsafe { CStr::from_ptr(buffer.as_ptr().cast()) }
            .to_str()
            .ok()?;
        return parse_major_minor(text);
    }

    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    {
        use std::ffi::CStr;
        // glibc exposes its loaded runtime version directly; wheels requiring
        // a newer baseline must not be selected for this host.
        // (Lấy phiên bản glibc đang nạp để từ chối wheel yêu cầu mới hơn.)
        // SAFETY: glibc static string, never freed; null-checked below.
        // (AN TOÀN: chuỗi tĩnh của glibc, không bao giờ free; kiểm tra null bên dưới.)
        let version = unsafe { libc::gnu_get_libc_version() };
        if version.is_null() {
            return None;
        }
        // SAFETY: non-null glibc static NUL-terminated string (see fn-level note).
        // (AN TOÀN: chuỗi tĩnh của glibc khác null, kết thúc NUL — xem ghi chú cấp hàm.)
        let text = unsafe { CStr::from_ptr(version) }.to_str().ok()?;
        return parse_major_minor(text);
    }

    // A musl build has no portable in-process version query here. Keep native
    // wheels fail-closed until a trusted musl version probe is available.
    // (Musl chưa có truy vấn phiên bản portable; wheel native vẫn bị từ chối.)
    #[allow(unreachable_code)]
    None
}

pub(in crate::protocols::pypi) fn parse_major_minor(value: &str) -> Option<(u64, u64)> {
    let mut parts = value.split('.');
    Some((parts.next()?.parse().ok()?, parts.next()?.parse().ok()?))
}
