use std::{borrow::Cow, convert::Infallible};

use crate::{SValue, scratch_expr::QuirkSink};

impl SValue {
    pub fn q_as_text<Q>(&self, _: &mut Q) -> Cow<'_, str>
    where
        Q: QuirkSink<Infallible>,
    {
        match self {
            Self::Text(t) => Cow::Borrowed(t),
            Self::Bool(true) => Cow::Borrowed("true"),
            Self::Bool(false) => Cow::Borrowed("false"),
            Self::Int(i) => Cow::Owned(i.to_string()),
            Self::Float(f) => Cow::Owned(f.to_string()),
        }
    }
}
