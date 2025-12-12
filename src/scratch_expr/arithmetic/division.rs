use crate::scratch_expr::{QuirkSink, SNumber, conversions::SNumberToFloatQ};

impl SNumber {
    /// Integers are automatically divided as floats (if the remainder != 0) but I think this
    /// shouldn't be considered a quirk as some people would actually like it (?)
    ///
    /// The division rules are not necessarily identical to IEEE 754.
    ///
    /// - n·m / m → n (integer division without remainder)
    ///
    /// If both numbers are finite integers and the division would have a remainder
    /// of 0 the numbers are divided as integers.
    /// Otherwise, the below table applys. p and q are arbitraty finite floats.
    ///
    /// (line / column is shown in cell)
    ///
    /// | `/`   |    | `NaN` | `+∞`  | `-∞`  |  +q  | -q   | `+0`  | `-0`  |
    /// | ----- | -- | ----- | ----- | ----- | ---- | ---- | ----- | ----- |
    /// |       |    |       |       |       |      |      |       |       |
    /// | `NaN` |    | `NaN` | `+0`  | `-0`  | `+0` | `-0` | `NaN` | `NaN` |
    /// | `+∞`  |    | `+∞`  | `NaN` | `NaN` | `+∞` | `-∞` | `+∞`  | `-∞`  |
    /// | `-∞`  |    | `-∞`  | `NaN` | `NaN` | `-∞` | `+∞` | `-∞`  | `+∞`  |
    /// |  +p   |    | `+∞`  | `+0`  | `-0`  |  p/q | -p/q | `+∞`  | `-∞`  |
    /// |  -p   |    | `-∞`  | `-0`  | `+0`  | -p/q |  p/q | `-∞`  | `+∞`  |
    /// | `+0`  |    | `NaN` | `+0`  | `-0`  | `+0` | `-0` | `NaN` | `NaN` |
    /// | `-0`  |    | `NaN` | `-0`  | `+0`  | `-0` | `+0` | `NaN` | `NaN` |
    pub fn q_div_numbers<Q>(&self, other: &SNumber, sink: &mut Q) -> SNumber
    where
        Q: QuirkSink<SNumberToFloatQ>,
    {
        if let (SNumber::Int(a), SNumber::Int(b)) = (self, other)
            && a.checked_rem(*b) == Some(0)
            && let Some(res) = a.checked_div(*b)
        {
            return SNumber::Int(res);
        }

        let a = self.q_as_float(sink);
        let b = other.q_as_float(sink);

        if a.is_nan() && b.is_nan() {
            return SNumber::NAN;
        }

        if (a == 0.0 || a == -0.0) && b.is_nan() {
            return Self::NAN;
        }
        if a.is_nan() && (b == 0.0 || b == -0.0) {
            return Self::NAN;
        }

        if b.is_nan() {
            return if a.is_sign_positive() {
                SNumber::INFINITY
            } else {
                SNumber::NEG_INFINITY
            };
        }
        if a.is_nan() && b.is_infinite() {
            return SNumber::Float(if b.is_sign_positive() { 0.0 } else { -0.0 });
        }
        if a.is_nan() && b.is_finite() {
            return SNumber::Float(if b.is_sign_positive() { 0.0 } else { -0.0 });
        }

        SNumber::Float(a / b)
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::SValue;
    use crate::scratch_expr::{int2reprs, ints2reprs};

    fn s_is_nan(s: crate::SValue) -> bool {
        if let crate::SValue::Float(f) = s
            && f.is_nan()
        {
            true
        } else {
            false
        }
    }

    #[test]
    fn test_division_by_nan() {
        assert_eq!(SValue::NAN, SValue::NAN / SValue::NAN);

        // finite, non-zero integer
        for x in 1..100 {
            for num in int2reprs(x) {
                assert_eq!(SValue::INFINITY, num / SValue::NAN);
            }
            for num in int2reprs(-x) {
                assert_eq!(SValue::NEG_INFINITY, num / SValue::NAN);
            }
        }
        // finite, non-zero float
        for num in &[1.23, 42.24, 3.9e12] {
            let neg_num = SValue::Float(-num);
            let num = SValue::Float(*num);
            assert_eq!(SValue::INFINITY, num.q_div_numbers(&SValue::NAN, &mut ()));
            assert_eq!(
                SValue::NEG_INFINITY,
                neg_num.q_div_numbers(&SValue::NAN, &mut ())
            );
        }
        let pos_zero = SValue::Float(0.0);
        let neg_zero = SValue::Float(-0.0);

        assert_eq!(SValue::NAN, pos_zero / SValue::NAN);
        assert_eq!(SValue::NAN, neg_zero / SValue::NAN);

        assert_eq!(SValue::INFINITY, SValue::INFINITY / SValue::NAN);
        assert_eq!(SValue::NEG_INFINITY, SValue::NEG_INFINITY / SValue::NAN);
    }

    #[test]
    fn test_division_with_text() {
        assert_eq!(SValue::NAN, (SValue::from("") / SValue::from("")));
        assert!(s_is_nan(SValue::from("") / SValue::from("abc")));
        assert!(s_is_nan(SValue::from("abc") / SValue::from("")));
        assert!(s_is_nan(SValue::from("abc") / SValue::from("abc")));
    }

    #[test]
    fn test_division_by_zero() {
        // 0 / 0 --> NaN
        for (za, zb) in ints2reprs(0, 0) {
            assert!(s_is_nan(za / zb));
        }

        // +1 / 0 --> +∞
        for (one, zero) in ints2reprs(1, 0) {
            assert_eq!(SValue::INFINITY, one / zero);
        }
        // -1 / 0 --> -∞
        for (negone, zero) in ints2reprs(-1, 0) {
            assert_eq!(SValue::NEG_INFINITY, negone / zero)
        }
        let pos_zero = SValue::Float(0.0);
        let neg_zero = SValue::Float(-0.0);

        assert_eq!(SValue::NAN, &SValue::NAN / &pos_zero);
        assert_eq!(SValue::NAN, &SValue::NAN / &neg_zero);

        // 1 / (+0) --> +∞
        // 1 / (-0) --> -∞
        for pos in int2reprs(1) {
            assert_eq!(SValue::INFINITY, &pos / &pos_zero);
            assert_eq!(SValue::NEG_INFINITY, &pos / &neg_zero);
        }
        // -1 / (+0) --> -∞
        // -1 / (-0) --> +∞
        for neg in int2reprs(-1) {
            assert_eq!(SValue::NEG_INFINITY, &neg / &pos_zero);
            assert_eq!(SValue::INFINITY, &neg / &neg_zero);
        }
        // ±0 / ±0 --> NaN
        for (a, b) in [pos_zero.clone(), neg_zero.clone()]
            .into_iter()
            .cartesian_product([pos_zero.clone(), neg_zero.clone()])
        {
            assert!(s_is_nan(a / b));
        }
        // -∞ / +0 --> -∞
        assert_eq!(
            SValue::NEG_INFINITY,
            SValue::NEG_INFINITY / pos_zero.clone()
        );
        // -∞ / -0 --> +∞
        assert_eq!(SValue::INFINITY, SValue::NEG_INFINITY / neg_zero.clone());
        // +∞ / +0 --> +∞
        assert_eq!(SValue::INFINITY, SValue::INFINITY / pos_zero.clone());
        // +∞ / -0 --> -∞
        assert_eq!(SValue::NEG_INFINITY, SValue::INFINITY / neg_zero.clone());
    }

    #[test]
    fn test_division_of_zero() {
        // 0 / +1 --> 0
        for (one, zero) in ints2reprs(1, 0) {
            assert_eq!(SValue::Int(0), zero / one);
        }
        // 0 / -1 --> -0
        for (negone, zero) in ints2reprs(-1, 0) {
            assert_eq!(SValue::Float(-0.0), zero / negone);
        }
        let pos_zero = SValue::Float(0.0);
        let neg_zero = SValue::Float(-0.0);

        // (+0) / 1 --> +0
        // (-0) / 1 --> -0
        for pos in int2reprs(1) {
            assert_eq!(pos_zero, &pos_zero / &pos);
            assert_eq!(neg_zero, &neg_zero / &pos);
        }
        // (+0) / -1 --> -0
        // (-0) / -1 --> +0
        for neg in int2reprs(-1) {
            assert_eq!(neg_zero, &pos_zero / &neg);
            assert_eq!(pos_zero, &neg_zero / &neg);
        }
    }
    #[test]
    fn test_division_by_infinity() {
        let pos_zero = SValue::Float(0.0);
        let neg_zero = SValue::Float(-0.0);
        assert_eq!(pos_zero, SValue::NAN / SValue::INFINITY);
        assert_eq!(SValue::NAN, SValue::INFINITY / SValue::INFINITY);
        assert_eq!(SValue::NAN, SValue::NEG_INFINITY / SValue::INFINITY);

        // finite, non-zero integer
        for x in 1..100 {
            for num in int2reprs(x) {
                assert_eq!(pos_zero, num / SValue::INFINITY);
            }
            for num in int2reprs(-x) {
                assert_eq!(neg_zero, num / SValue::INFINITY);
            }
        }
        assert_eq!(pos_zero, &pos_zero / &SValue::INFINITY);
        assert_eq!(neg_zero, &neg_zero / &SValue::INFINITY);
    }
    #[test]
    fn test_division_by_neg_infinity() {
        let pos_zero = SValue::Float(0.0);
        let neg_zero = SValue::Float(-0.0);
        assert_eq!(pos_zero, SValue::NAN / SValue::NEG_INFINITY);
        assert_eq!(SValue::NAN, SValue::INFINITY / SValue::NEG_INFINITY);
        assert_eq!(SValue::NAN, SValue::NEG_INFINITY / SValue::NEG_INFINITY);

        // finite, non-zero integer
        for x in 1..100 {
            for num in int2reprs(x) {
                assert_eq!(neg_zero, num / SValue::NEG_INFINITY);
            }
            for num in int2reprs(-x) {
                assert_eq!(pos_zero, num / SValue::NEG_INFINITY);
            }
        }
        assert_eq!(neg_zero, &pos_zero / &SValue::NEG_INFINITY);
        assert_eq!(pos_zero, &neg_zero / &SValue::NEG_INFINITY);
    }

    #[test]
    fn test_division_by_finite() {
        let pos_zero = SValue::Float(0.0);
        let neg_zero = SValue::Float(-0.0);

        // finite, non-zero integer
        for x in 1..100 {
            for num in int2reprs(x) {
                assert_eq!(pos_zero, &SValue::NAN / &num);
                assert_eq!(SValue::INFINITY, &SValue::INFINITY / &num);
                assert_eq!(SValue::NEG_INFINITY, &SValue::NEG_INFINITY / &num);
            }
            for num in int2reprs(-x) {
                assert_eq!(neg_zero, &SValue::NAN / &num);
                assert_eq!(SValue::NEG_INFINITY, &SValue::INFINITY / &num);
                assert_eq!(SValue::INFINITY, &SValue::NEG_INFINITY / &num);
            }
        }
    }
}
