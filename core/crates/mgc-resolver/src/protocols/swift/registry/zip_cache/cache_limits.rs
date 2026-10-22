//! Defaults and environment overrides for retained Swift archive memory.
//! Mặc định và biến môi trường cho bộ nhớ archive Swift được giữ lại.

const DEFAULT_MAX_BYTES: usize = 128 * 1024 * 1024;
const HARD_MAX_BYTES: usize = 512 * 1024 * 1024;
const DEFAULT_MAX_ENTRIES: usize = 64;
const HARD_MAX_ENTRIES: usize = 512;

#[derive(Clone, Copy, Debug)]
pub(super) struct SwiftZipCacheLimits {
    pub(super) max_bytes: usize,
    pub(super) max_entries: usize,
}

impl SwiftZipCacheLimits {
    pub(super) fn from_env() -> Self {
        Self {
            max_bytes: read_limit(
                "MGC_SWIFT_REGISTRY_CACHE_MAX_BYTES",
                DEFAULT_MAX_BYTES,
                HARD_MAX_BYTES,
            ),
            max_entries: read_limit(
                "MGC_SWIFT_REGISTRY_CACHE_MAX_ENTRIES",
                DEFAULT_MAX_ENTRIES,
                HARD_MAX_ENTRIES,
            ),
        }
    }
}

impl Default for SwiftZipCacheLimits {
    fn default() -> Self {
        Self {
            max_bytes: DEFAULT_MAX_BYTES,
            max_entries: DEFAULT_MAX_ENTRIES,
        }
    }
}

fn read_limit(name: &str, default: usize, hard_max: usize) -> usize {
    match std::env::var(name) {
        Ok(raw) => match raw.parse::<usize>() {
            Ok(value) if value <= hard_max => value,
            Ok(_) => {
                eprintln!("WARNING: {name} exceeds its hard limit; using {hard_max}");
                hard_max
            }
            Err(_) => invalid_limit(name, default),
        },
        Err(std::env::VarError::NotPresent) => default,
        Err(std::env::VarError::NotUnicode(_)) => {
            eprintln!("WARNING: {name} is not valid Unicode; using the default value");
            default
        }
    }
}

fn invalid_limit(name: &str, default: usize) -> usize {
    eprintln!("WARNING: {name} is invalid; using the default value");
    default
}
