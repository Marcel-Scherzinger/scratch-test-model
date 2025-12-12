use super::{QuirkSink, SValue};

#[allow(clippy::enum_variant_names)]
pub enum NthLetterOfTextQ {
    PositionIsNotInt(SValue),
    PositionZero,
    PositionNegative(i64),
    PositionTooBig(i64),
}
impl SValue {
    fn contains_text(&self, maybe_contained: &SValue) -> bool {
        self.as_text().contains(maybe_contained.as_text().as_ref())
    }
    fn nth_letter_of_me<Q>(&self, one_based_n: &SValue, sink: &mut Q) -> SValue
    where
        Q: QuirkSink<NthLetterOfTextQ>,
    {
        let letter_pos = one_based_n.as_int();
        let string = self.as_text();
        if !one_based_n.is_int() {
            sink.put(NthLetterOfTextQ::PositionIsNotInt(one_based_n.clone()));
        } else if letter_pos < 0 {
            sink.put(NthLetterOfTextQ::PositionNegative(letter_pos));
        } else if letter_pos == 0 {
            sink.put(NthLetterOfTextQ::PositionZero);
        } else if letter_pos as usize <= string.len() {
            let zero_based = (letter_pos - 1) as usize;
            if let Some(d) = string.chars().nth(zero_based) {
                return SValue::Text(d.to_string().into());
            }
        } else {
            sink.put(NthLetterOfTextQ::PositionTooBig(letter_pos));
        }

        SValue::Text("".into())
    }
}
