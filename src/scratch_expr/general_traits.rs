use std::{convert::Infallible, str::FromStr};

use crate::ARc;
use crate::error::{FormatError, IntegerOutOfBounds};

use super::SValue;

impl FromStr for SValue {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(float) = s.parse() {
            if s.contains(".") || float < (i64::MIN as f64) || (i64::MAX as f64) < float {
                Ok(Self::Float(float))
            } else if let Ok(int) = s.parse() {
                Ok(Self::Int(int))
            } else {
                Ok(Self::Text(s.into()))
            }
        } else {
            Ok(Self::Text(s.into()))
        }
    }
}

impl From<String> for SValue {
    fn from(value: String) -> Self {
        Self::Text(value.into())
    }
}

impl From<ARc<str>> for SValue {
    fn from(value: ARc<str>) -> Self {
        Self::Text(value)
    }
}
impl From<i64> for SValue {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}
impl From<f64> for SValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}
impl TryFrom<u64> for SValue {
    type Error = FormatError;

    fn try_from(value: u64) -> Result<SValue, Self::Error> {
        Ok(Self::Int(value.try_into().map_err(|_| {
            FormatError::IntegerBounds(IntegerOutOfBounds)
        })?))
    }
}
impl TryFrom<serde_json::Number> for SValue {
    type Error = FormatError;

    fn try_from(value: serde_json::Number) -> Result<SValue, Self::Error> {
        Ok(if let Some(n) = value.as_f64() {
            Self::Float(n)
        } else if let Some(n) = value.as_i64() {
            Self::Int(n)
        } else if let Some(n) = value.as_u64() {
            Self::Int(
                n.try_into()
                    .map_err(|_| FormatError::IntegerBounds(IntegerOutOfBounds))?,
            )
        } else {
            Self::Int(0)
        })
    }
}
