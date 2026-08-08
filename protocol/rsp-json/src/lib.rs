#![forbid(unsafe_code)]
//! Deterministic canonical JSON encoding for RSP semantic resources.
//!
//! RSP v1 uses a deliberately restricted JSON profile: object keys are ordered
//! by UTF-16 code units, strings use JSON escaping, arrays preserve order, and
//! floating-point / unsafe integer values are rejected. Large protocol values
//! such as revisions and content lengths are represented as decimal strings by
//! their semantic types before reaching this encoder.

use core::{cmp::Ordering, fmt};
use serde::Serialize;
use serde_json::Value;

const MAX_SAFE_JSON_INTEGER: u64 = 9_007_199_254_740_991;

#[derive(Debug)]
pub enum CanonicalJsonError {
    Serialize(serde_json::Error),
    NonCanonicalNumber,
}

impl fmt::Display for CanonicalJsonError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialize(error) => write!(formatter, "JSON serialization failed: {error}"),
            Self::NonCanonicalNumber => formatter.write_str(
                "RSP canonical JSON forbids floating-point and integers outside the exact binary64 range",
            ),
        }
    }
}

impl std::error::Error for CanonicalJsonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serialize(error) => Some(error),
            Self::NonCanonicalNumber => None,
        }
    }
}

impl From<serde_json::Error> for CanonicalJsonError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialize(value)
    }
}

/// Serializes an RSP resource to deterministic canonical UTF-8 JSON bytes.
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>, CanonicalJsonError>
where
    T: Serialize,
{
    let value = serde_json::to_value(value)?;
    let mut output = Vec::new();
    write_value(&value, &mut output)?;
    Ok(output)
}

/// Serializes an RSP resource to deterministic canonical JSON text.
pub fn to_string<T>(value: &T) -> Result<String, CanonicalJsonError>
where
    T: Serialize,
{
    Ok(String::from_utf8(to_vec(value)?).expect("canonical JSON writer only emits UTF-8"))
}

fn write_value(value: &Value, output: &mut Vec<u8>) -> Result<(), CanonicalJsonError> {
    match value {
        Value::Null => output.extend_from_slice(b"null"),
        Value::Bool(true) => output.extend_from_slice(b"true"),
        Value::Bool(false) => output.extend_from_slice(b"false"),
        Value::Number(number) => {
            if let Some(value) = number.as_u64() {
                if value > MAX_SAFE_JSON_INTEGER {
                    return Err(CanonicalJsonError::NonCanonicalNumber);
                }
                output.extend_from_slice(value.to_string().as_bytes());
            } else if let Some(value) = number.as_i64() {
                if value.unsigned_abs() > MAX_SAFE_JSON_INTEGER {
                    return Err(CanonicalJsonError::NonCanonicalNumber);
                }
                output.extend_from_slice(value.to_string().as_bytes());
            } else {
                return Err(CanonicalJsonError::NonCanonicalNumber);
            }
        }
        Value::String(value) => output.extend_from_slice(serde_json::to_string(value)?.as_bytes()),
        Value::Array(values) => {
            output.push(b'[');
            for (index, value) in values.iter().enumerate() {
                if index != 0 {
                    output.push(b',');
                }
                write_value(value, output)?;
            }
            output.push(b']');
        }
        Value::Object(values) => {
            let mut entries: Vec<_> = values.iter().collect();
            entries.sort_by(|(left, _), (right, _)| utf16_cmp(left, right));

            output.push(b'{');
            for (index, (key, value)) in entries.into_iter().enumerate() {
                if index != 0 {
                    output.push(b',');
                }
                output.extend_from_slice(serde_json::to_string(key)?.as_bytes());
                output.push(b':');
                write_value(value, output)?;
            }
            output.push(b'}');
        }
    }

    Ok(())
}

fn utf16_cmp(left: &str, right: &str) -> Ordering {
    left.encode_utf16().cmp(right.encode_utf16())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_json::json;

    #[test]
    fn object_keys_are_sorted_by_utf16_code_units() {
        let value = json!({
            "\u{1f600}": 1,
            "\u{20ac}": 2,
            "a": 3
        });

        assert_eq!(to_string(&value).unwrap(), "{\"a\":3,\"😀\":1,\"€\":2}");
    }

    #[test]
    fn unsafe_numbers_are_rejected() {
        assert!(matches!(
            to_vec(&json!(9_007_199_254_740_992_u64)),
            Err(CanonicalJsonError::NonCanonicalNumber)
        ));
        assert!(matches!(
            to_vec(&json!(1.5)),
            Err(CanonicalJsonError::NonCanonicalNumber)
        ));
    }

    #[test]
    fn serialization_is_stable_for_structs() {
        #[derive(Serialize)]
        struct Fixture<'a> {
            z: &'a str,
            a: &'a str,
        }

        assert_eq!(
            to_string(&Fixture { z: "last", a: "first" }).unwrap(),
            "{\"a\":\"first\",\"z\":\"last\"}"
        );
    }
}
