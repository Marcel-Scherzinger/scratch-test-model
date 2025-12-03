use crate::{ARc, SValue, scratch_expr::QuirkSink};

pub enum SValueToIntQ {
    TextIsFloatNotInt(f64),
    TextNotANumber(ARc<str>),
    BoolNotANumber(bool),
    FloatSmallerThanIntMin(f64),
    FloatBiggerThanIntMax(f64),
    NaN,
    PosInfty,
    NegInfty,
}

impl SValue {
    pub fn q_as_int<Q>(&self, sink: &mut Q) -> i64
    where
        Q: QuirkSink<SValueToIntQ>,
    {
        match self {
            Self::Int(i) => *i,
            Self::Text(t) => {
                if let Ok(i) = t.parse() {
                    i
                } else if let Ok(f) = t.parse::<f64>() {
                    sink.put(SValueToIntQ::TextIsFloatNotInt(f));
                    Self::Float(f).q_as_int(sink)
                } else {
                    sink.put(SValueToIntQ::TextNotANumber(t.clone()));
                    0
                }
            }
            Self::Bool(b) => {
                sink.put(SValueToIntQ::BoolNotANumber(*b));
                if *b { 1 } else { 0 }
            }
            Self::Float(f) => {
                let f = *f;
                if f.is_finite() {
                    let f = f.round();
                    if i64::MIN as f64 <= f {
                        if f <= i64::MAX as f64 {
                            f as i64
                        } else {
                            // value doesn't fit into i64
                            sink.put(SValueToIntQ::FloatBiggerThanIntMax(f));
                            i64::MAX
                        }
                    } else {
                        // value doesn't fit into i64
                        sink.put(SValueToIntQ::FloatSmallerThanIntMin(f));
                        i64::MIN
                    }
                } else if f.is_nan() {
                    // scratch treats nan as 0
                    sink.put(SValueToIntQ::NaN);
                    0
                } else if f.is_sign_positive() {
                    // positive infinity
                    // WARNING: behaviour will be different from scratch
                    sink.put(SValueToIntQ::PosInfty);
                    i64::MAX
                } else {
                    // negative infinity
                    // WARNING: behaviour will be different from scratch
                    sink.put(SValueToIntQ::NegInfty);
                    i64::MIN
                }
            }
        }
    }
}
