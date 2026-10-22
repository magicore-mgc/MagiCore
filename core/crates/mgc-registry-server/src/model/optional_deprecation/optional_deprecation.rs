//! Accept npm registry's string, boolean, or null `deprecated` representation.
//! Nhận biểu diễn `deprecated` dạng string, bool hoặc null từ npm registry.

use serde::Deserialize;

pub(in crate::model) fn de_optional_string<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct OptStr;
    impl<'de> serde::Deserialize<'de> for OptStr {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            #[derive(serde::Deserialize)]
            #[serde(untagged)]
            #[allow(dead_code)]
            enum V {
                S(String),
                B(bool),
            }
            match V::deserialize(deserializer)? {
                V::S(_) | V::B(_) => Ok(OptStr),
            }
        }
    }
    Ok(Option::<OptStr>::deserialize(deserializer)?.map(|_| String::new()))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
