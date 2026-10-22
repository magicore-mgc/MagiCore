//! Closed-set loader semantics and serde representation.
//! Semantics loader tập đóng và biểu diễn serde.

/// Loader semantics for a compilation key (P0-1): the same text compiles
/// differently per loader, so the loader is part of the key's identity —
/// and it must be a CLOSED SET, not a free-form String ("garbage-loader"
/// used to pass validation and fork the cache namespace).
///
/// Semantics loader cho compilation key (P0-1): cùng text biên dịch khác
/// nhau theo loader nên loader là một phần identity của key — và nó phải
/// là tập ĐÓNG, không phải String tự do ("garbage-loader" từng lọt qua
/// validation và rẽ nhánh namespace cache).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Loader {
    Ts,
    Tsx,
    Jsx,
    Js,
    Mjs,
}

impl Loader {
    /// Map a file extension to its loader — the ONLY sanctioned way the
    /// dev server derives loader identity.
    /// (Ánh xạ đuôi file sang loader — cách DUY NHẤT được phép để dev
    /// server suy ra identity loader.)
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_ascii_lowercase().as_str() {
            "ts" => Some(Self::Ts),
            "tsx" => Some(Self::Tsx),
            "jsx" => Some(Self::Jsx),
            "js" => Some(Self::Js),
            "mjs" => Some(Self::Mjs),
            _ => None,
        }
    }
}

impl std::fmt::Display for Loader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Ts => "ts",
            Self::Tsx => "tsx",
            Self::Jsx => "jsx",
            Self::Js => "js",
            Self::Mjs => "mjs",
        })
    }
}

impl std::str::FromStr for Loader {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_extension(s)
            .ok_or_else(|| format!("unknown loader '{s}' (expected ts|tsx|jsx|js|mjs)"))
    }
}

/// Serde via the Display/FromStr pair: records carry the lowercase tag,
/// and a record whose loader is NOT in the closed set fails to parse —
/// a garbage-loader record can never become a CompilationKey again.
/// (Serde qua cặp Display/FromStr: record mang tag lowercase, record có
/// loader NGOÀI tập đóng thì parse fail — record garbage-loader không
/// bao giờ thành CompilationKey được nữa.)
impl serde::Serialize for Loader {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Loader {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        s.parse::<Self>().map_err(serde::de::Error::custom)
    }
}
