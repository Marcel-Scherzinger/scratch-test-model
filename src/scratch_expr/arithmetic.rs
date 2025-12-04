use crate::{
    SValue,
    scratch_expr::{
        QuirkSink, SNumber,
        conversions::{SNumberToFloatQ, SValueToNumberQ},
    },
};

/// An addition of two integers would overflow so they're treated as
/// floating point numbers for addition, a float is then produces instead of an int.
pub struct IntegerAddWouldOverflow;

impl SValue {
    pub fn q_add_numbers<Q>(&self, other: &SValue, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SValueToNumberQ>
            + QuirkSink<SNumberToFloatQ>
            + QuirkSink<IntegerAddWouldOverflow>,
    {
        let a = self.q_as_number(sink);
        let b = other.q_as_number(sink);
        a.q_add_numbers(&b, sink)
    }
}
impl SNumber {
    pub fn q_add_numbers<Q>(&self, other: &SNumber, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SNumberToFloatQ> + QuirkSink<IntegerAddWouldOverflow>,
    {
        match (self, other) {
            (SNumber::Float(_), _) | (_, SNumber::Float(_)) => {
                let a: f64 = self.q_as_float(sink);
                let b: f64 = other.q_as_float(sink);
                // TODO: WARNING: unsure if this could panic
                SNumber::Float(a + b)
            }
            (SNumber::Int(a), SNumber::Int(b)) => {
                let checked_sum = a.checked_add(*b);
                if let Some(sum) = checked_sum {
                    SNumber::Int(sum)
                } else {
                    sink.put(IntegerAddWouldOverflow);
                    SNumber::Float((*a as f64) + (*b as f64))
                }
            }
        }
    }
}
