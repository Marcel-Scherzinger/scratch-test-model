use std::borrow::Cow;

use crate::{SValue, scratch_expr::SNumber};

impl SNumber {
    pub(crate) const fn svalue(self) -> SValue {
        match self {
            Self::Int(i) => SValue::Int(i),
            Self::Float(f) => SValue::Float(f),
        }
    }
}

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
        self.q_as_bool(&mut ())
    }

    pub fn as_text(&self) -> Cow<'_, str> {
        self.q_as_text(&mut ())
    }
    // WARNING: for over-/underflow the behaviour is different from Scratch
    pub fn as_int(&self) -> i64 {
        self.q_as_int(&mut ())
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
