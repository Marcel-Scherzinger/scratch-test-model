use crate::SValue;
use itertools::Itertools;

/// This method is used to perform checks/tests for all ways an integer
/// number could be provided as a Scratch number.
///
/// - Directly as an integer ([`SValue::Int`])
/// - As a float ending with `.0` ([`SValue::Float`])
/// - As a textual representation of the digits (without `.0`), ([`SValue::Text`])
/// - As a textual representation of the digits with trailing `.0`, ([`SValue::Text`])
///
/// Those four representations are returned as a [`Vec<SValue>`]
pub(crate) fn int2reprs(a: i64) -> Vec<SValue> {
    vec![
        SValue::Int(a),
        SValue::Float(a as f64),
        SValue::Text(a.to_string().into()),
        SValue::Text((a as f64).to_string().into()),
    ]
}

/// This method returns all variations these two numbers could be
/// represented as Scratch values.
///
/// It's the cartesian product of [`int2reprs`]
pub(crate) fn ints2reprs(a: i64, b: i64) -> impl Iterator<Item = (SValue, SValue)> {
    int2reprs(a).into_iter().cartesian_product(int2reprs(b))
}
