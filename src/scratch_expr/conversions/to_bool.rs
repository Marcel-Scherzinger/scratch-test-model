use crate::{SValue, scratch_expr::QuirkSink};

pub enum SValueToBoolQ {
    EmptyStringTreatedAsFalse,
    NonEmptyStringTreatedAsTrue,
    ZeroToFalse,
    OneToTrue,
    IntToBoolNotLossless(i64),
    FloatToBoolNotLossless(f64),
}

impl SValue {
    /// Converts the value to a boolean according to Scratch logic.
    /// Occuring behaviour differences Scratch implements to avoid errors
    /// will be reported to the sink.
    ///
    /// As expression blocks can't be placed in comparison contexts
    /// these quriks should never occur because of a user's program
    ///
    /// For completness they are reported.
    pub fn q_as_bool<Q>(&self, sink: &mut Q) -> bool
    where
        Q: QuirkSink<SValueToBoolQ>,
    {
        match self {
            Self::Text(t) => {
                if t.as_ref() == "" {
                    sink.put(SValueToBoolQ::EmptyStringTreatedAsFalse);
                    false
                } else if t.as_ref() == "false" {
                    false
                } else if t.as_ref() == "true" {
                    true
                } else {
                    sink.put(SValueToBoolQ::NonEmptyStringTreatedAsTrue);
                    true
                }
            }
            Self::Bool(b) => *b,
            Self::Int(0) | Self::Float(0.0) => {
                sink.put(SValueToBoolQ::ZeroToFalse);
                false
            }
            Self::Int(1) | Self::Float(1.0) => {
                sink.put(SValueToBoolQ::OneToTrue);
                true
            }
            Self::Int(i) => {
                sink.put(SValueToBoolQ::IntToBoolNotLossless(*i));
                true
            }
            Self::Float(f) => {
                sink.put(SValueToBoolQ::FloatToBoolNotLossless(*f));
                true
            }
        }
    }
}
