//! Stream and validate the top-level esbuild metafile shape.
//! Đọc streaming và kiểm shape cấp cao nhất của metafile esbuild.

use std::{cell::Cell, cell::RefCell, collections::HashMap, fmt};

use serde::de::{self, DeserializeSeed, IgnoredAny, MapAccess, Visitor};

use super::super::error::MetafileInputsError;

/// Parse metadata without allocating a duplicate JSON value tree.
/// Parse metadata mà không cấp phát thêm cây JSON trùng lặp.
pub(in crate::bundler::dependency_cache::build::metafile_inputs) fn validate_document<F>(
    metafile: &str,
    input_validator: F,
) -> Result<(), MetafileInputsError>
where
    F: Fn(&str) -> Result<(), MetafileInputsError> + Copy,
{
    let failure = RefCell::new(None);
    let inputs_seen = Cell::new(false);
    let mut deserializer = serde_json::Deserializer::from_str(metafile);
    let seed = MetafileSeed {
        input_validator,
        inputs_seen: &inputs_seen,
        failure: &failure,
    };

    if seed.deserialize(&mut deserializer).is_err() || deserializer.end().is_err() {
        return Err(failure
            .into_inner()
            .unwrap_or(MetafileInputsError::InvalidMetadata));
    }

    Ok(())
}

struct MetafileSeed<'a, F> {
    input_validator: F,
    inputs_seen: &'a Cell<bool>,
    failure: &'a RefCell<Option<MetafileInputsError>>,
}

impl<'de, F> DeserializeSeed<'de> for MetafileSeed<'_, F>
where
    F: Fn(&str) -> Result<(), MetafileInputsError> + Copy,
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(MetafileVisitor {
            input_validator: self.input_validator,
            inputs_seen: self.inputs_seen,
            failure: self.failure,
        })
    }
}

struct MetafileVisitor<'a, F> {
    input_validator: F,
    inputs_seen: &'a Cell<bool>,
    failure: &'a RefCell<Option<MetafileInputsError>>,
}

impl<'de, F> Visitor<'de> for MetafileVisitor<'_, F>
where
    F: Fn(&str) -> Result<(), MetafileInputsError> + Copy,
{
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("an esbuild metafile object with an inputs map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        while let Some(key) = map.next_key::<String>()? {
            if key == "inputs" {
                if self.inputs_seen.replace(true) {
                    self.failure
                        .replace(Some(MetafileInputsError::InvalidMetadata));
                    return Err(de::Error::custom("duplicate inputs map"));
                }
                map.next_value_seed(InputsSeed {
                    input_validator: self.input_validator,
                    failure: self.failure,
                })?;
            } else {
                map.next_value::<IgnoredAny>()?;
            }
        }

        if !self.inputs_seen.get() {
            self.failure
                .replace(Some(MetafileInputsError::InvalidMetadata));
            return Err(de::Error::custom("missing inputs map"));
        }

        Ok(())
    }
}

struct InputsSeed<'a, F> {
    input_validator: F,
    failure: &'a RefCell<Option<MetafileInputsError>>,
}

impl<'de, F> DeserializeSeed<'de> for InputsSeed<'_, F>
where
    F: Fn(&str) -> Result<(), MetafileInputsError> + Copy,
{
    type Value = ();

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(InputsVisitor {
            input_validator: self.input_validator,
            failure: self.failure,
        })
    }
}

struct InputsVisitor<'a, F> {
    input_validator: F,
    failure: &'a RefCell<Option<MetafileInputsError>>,
}

impl<'de, F> Visitor<'de> for InputsVisitor<'_, F>
where
    F: Fn(&str) -> Result<(), MetafileInputsError> + Copy,
{
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a non-empty esbuild input map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut input_count = 0usize;
        while let Some(input) = map.next_key::<String>()? {
            input_count += 1;
            if let Err(error) = (self.input_validator)(&input) {
                self.failure.replace(Some(error));
                return Err(de::Error::custom("input path rejected"));
            }
            map.next_value::<HashMap<String, IgnoredAny>>()?;
        }

        if input_count == 0 {
            self.failure
                .replace(Some(MetafileInputsError::InvalidMetadata));
            return Err(de::Error::custom("empty inputs map"));
        }

        Ok(())
    }
}
