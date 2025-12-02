use crate::{ARc, SValue, scratch_expr::QuirkSink};

pub enum SValueToFloatQ {
    BoolNotANumber(bool),
    TextNotANumber(ARc<str>),
    FloatToIntNotLossless,
}

impl SValue {
    pub fn q_as_float<Q>(&self, sink: &mut Q) -> f64
    where
        Q: QuirkSink<SValueToFloatQ>,
    {
        match self {
            Self::Bool(b) => {
                sink.put(SValueToFloatQ::BoolNotANumber(*b));
                if *b { 1.0 } else { 0.0 }
            }
            Self::Text(t) => {
                if let Ok(f) = t.parse() {
                    f
                } else {
                    sink.put(SValueToFloatQ::TextNotANumber(t.clone()));
                    0.0
                }
            }
            Self::Int(i1) => {
                let f: f64 = (*i1) as f64;
                let i2 = f as i64;
                if *i1 != i2 {
                    sink.put(SValueToFloatQ::FloatToIntNotLossless);
                }
                f
            }
            Self::Float(f) => *f,
        }
    }
}
