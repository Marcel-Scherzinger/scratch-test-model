mod as_is_type;
mod general_traits;
mod svalue;

use std::rc::Rc;

/// This should model a Scratch value.
/// Scratch treats texts that are non-numeric as the number `0` and also stores numbers
/// inside of arithmetic expressions as texts, at least sometimes.
///
/// So it is useful to have a type that mimics this implicit conversions
/// behaviour.
#[derive(derive_more::Debug, Clone, derive_more::Display, PartialEq)]
pub enum SValue {
    #[debug("{_0:?}")]
    Text(Rc<str>),
    #[debug("{_0:?}")]
    Int(i64),
    #[debug("{_0:?}")]
    Float(f64),
    #[debug("{_0:?}")]
    Bool(bool),
}

#[derive(Debug, thiserror::Error, PartialEq)]
#[error("integer out of bounds")]
pub struct IntegerOutOfBounds;
