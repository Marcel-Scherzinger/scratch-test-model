use std::borrow::Cow;

use crate::SValue;

impl SValue {
    pub fn is_float(&self) -> bool {
        match self {
            Self::Float(_) => true,
            Self::Int(_) => false,
            Self::Text(text) => text.contains(".") && text.parse::<f64>().is_ok(),
            Self::Bool(_) => false, // bools fit into ints
        }
    }
    pub fn is_int(&self) -> bool {
        match self {
            Self::Float(_) => false,
            Self::Int(_) => true,
            Self::Text(text) => text.parse::<i64>().is_ok(),
            Self::Bool(_) => true, // bools fit into ints
        }
    }
    pub fn as_bool(&self) -> bool {
        match self {
            Self::Int(i) => i != &0,
            Self::Float(f) => f != &0.0,
            Self::Text(t) => {
                if t.as_ref() == "" || t.as_ref() == "false" {
                    false
                } else if t.as_ref() == "true" {
                    true
                } else {
                    // this behaviour is unsure as it shouldn't occur in Scratch
                    true
                }
            }
            Self::Bool(b) => *b,
        }
    }

    pub fn as_text(&self) -> Cow<'_, str> {
        match &self {
            Self::Text(t) => Cow::Borrowed(t),
            Self::Int(i) => Cow::Owned(i.to_string()),
            Self::Float(f) => Cow::Owned(f.to_string()),
            Self::Bool(true) => Cow::Borrowed("true"),
            Self::Bool(false) => Cow::Borrowed("false"),
        }
    }
    // WARNING: for over-/underflow the behaviour is different from Scratch
    pub fn as_int(&self) -> i64 {
        match &self {
            Self::Text(t) => t.parse().unwrap_or(0),
            Self::Int(i) => *i,
            Self::Float(f) => {
                let f = *f;
                if f.is_finite() {
                    if i64::MIN as f64 <= f && f <= i64::MAX as f64 {
                        f.round() as i64
                    } else {
                        // value doesn't fit into i64
                        0
                    }
                } else if f.is_nan() {
                    // scratch treats nan as 0
                    0
                } else if f.is_sign_positive() {
                    // positive infinity
                    // WARNING: behaviour will be different from scratch
                    i64::MAX
                } else {
                    // negative infinity
                    // WARNING: behaviour will be different from scratch
                    i64::MIN
                }
            }
            Self::Bool(true) => 1,
            Self::Bool(false) => 0,
        }
    }
    pub fn as_float(&self) -> f64 {
        match &self {
            Self::Text(t) => t.parse().unwrap_or(0.0),
            Self::Int(i) => *i as f64, // WARNING: precision loss?
            Self::Float(f) => *f,
            Self::Bool(true) => 1.0,
            Self::Bool(false) => 0.0,
        }
    }
}
