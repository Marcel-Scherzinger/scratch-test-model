use crate::{
    ARc, SValue,
    scratch_expr::{QuirkSink, SNumber},
};

pub enum SValueToNumberQ {
    BoolNotANumber(bool),
    TextNotANumber(ARc<str>),
}

impl SValue {
    pub fn q_as_number<Q>(&self, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SValueToNumberQ>,
    {
        match self {
            Self::Bool(b) => {
                sink.put(SValueToNumberQ::BoolNotANumber(*b));
                if *b { SNumber::Int(1) } else { SNumber::Int(0) }
            }
            Self::Text(t) => {
                if let Ok(i) = t.parse() {
                    SNumber::Int(i)
                } else if let Ok(f) = t.parse() {
                    SNumber::Float(f)
                } else {
                    sink.put(SValueToNumberQ::TextNotANumber(t.clone()));
                    SNumber::Int(0)
                }
            }
            Self::Int(i) => SNumber::Int(*i),
            Self::Float(f) => SNumber::Float(*f),
        }
    }
}
