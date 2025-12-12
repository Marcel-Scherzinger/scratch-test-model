mod arithmetic;
mod as_is_type;
mod conversions;
mod general_traits;
mod quirk_sink;
mod svalue;
mod texts;
mod utils;

#[allow(unused)]
pub(crate) use utils::{int2reprs, ints2reprs};

pub use conversions::{SValueToBoolQ, SValueToFloatQ, SValueToIntQ};
pub use quirk_sink::{QuirkSink, ScopableQuirkSink};

use crate::ARc;

/// Only used in a few amount of cases where it is clear that neither booleans nor
/// texts can occur
#[derive(derive_more::Debug, Clone, derive_more::Display, PartialEq)]
pub enum SNumber {
    #[debug("{_0:?}")]
    Int(i64),
    #[debug("{_0:?}")]
    Float(f64),
}

impl SNumber {
    #[allow(unused)]
    pub(crate) const NAN: Self = Self::Float(f64::NAN);
    pub(crate) const INFINITY: Self = Self::Float(f64::INFINITY);
    pub(crate) const NEG_INFINITY: Self = Self::Float(f64::NEG_INFINITY);
}

/// This should model a Scratch value.
/// Scratch treats texts that are non-numeric as the number `0` and also stores numbers
/// inside of arithmetic expressions as texts, at least sometimes.
///
/// So it is useful to have a type that mimics this implicit conversions
/// behaviour.
#[derive(derive_more::Debug, Clone, derive_more::Display, PartialEq)]
pub enum SValue {
    #[debug("{_0:?}")]
    Text(ARc<str>),
    #[debug("{_0:?}")]
    Int(i64),
    #[debug("{_0:?}")]
    Float(f64),
    #[debug("{_0:?}")]
    Bool(bool),
}


impl SValue {
    pub(crate) const NAN: Self = Self::Float(f64::NAN);
    pub(crate) const INFINITY: Self = Self::Float(f64::INFINITY);
    pub(crate) const NEG_INFINITY: Self = Self::Float(f64::NEG_INFINITY);
}

#[derive(Debug, thiserror::Error, PartialEq)]
#[error("integer out of bounds")]
pub struct IntegerOutOfBounds;
