use crate::SValue;

impl SValue {
    /// Converts the given [usize] to an integer ([i64]) or yields [i64::MAX]
    pub fn int_or_max(num: usize) -> Self {
        if let Ok(v) = num.try_into() {
            Self::Int(v)
        } else {
            Self::Int(i64::MAX)
        }
    }

    pub fn scratch_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Text(a), Self::Text(b)) => a == b,
            (Self::Int(a), Self::Int(b)) => a == b,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a == b,

            (Self::Float(_), Self::Int(_)) | (Self::Int(_), Self::Float(_)) => {
                self.as_float() == other.as_float()
            }
            (Self::Text(_), Self::Int(_)) | (Self::Int(_), Self::Text(_)) => {
                self.as_int() == other.as_int()
            }
            (Self::Text(_), Self::Float(_)) | (Self::Float(_), Self::Text(_)) => {
                self.as_float() == other.as_float()
            }
            (Self::Text(t), Self::Bool(b)) | (Self::Bool(b), Self::Text(t)) => {
                if *b {
                    t.as_ref() == "true"
                } else {
                    t.as_ref() == "false"
                }
            }
            (Self::Int(_), Self::Bool(_)) | (Self::Bool(_), Self::Int(_)) => {
                self.as_int() == other.as_int()
            }
            (Self::Float(_), Self::Bool(_)) | (Self::Bool(_), Self::Float(_)) => {
                self.as_float() == other.as_float()
            }
        }
    }

    pub fn same_numbers_wrap_op(
        &self,
        other: &SValue,
        on_int: impl FnOnce(i64, i64) -> i64,
        on_float: impl FnOnce(f64, f64) -> f64,
    ) -> Self {
        self.same_numbers_op(
            other,
            |a, b| Self::Int(on_int(a, b)),
            |a, b| Self::Float(on_float(a, b)),
        )
    }
    pub fn same_numbers_op<O>(
        &self,
        other: &SValue,
        on_int: impl FnOnce(i64, i64) -> O,
        on_float: impl FnOnce(f64, f64) -> O,
    ) -> O {
        match (self, other) {
            (Self::Float(_), _) | (_, Self::Float(_)) => {
                on_float(self.as_float(), other.as_float())
            }
            (Self::Text(text), Self::Int(_)) if !text.contains(".") => {
                on_int(self.as_int(), other.as_int())
            }
            (Self::Int(_), Self::Text(text)) if !text.contains(".") => {
                on_int(self.as_int(), other.as_int())
            }
            (Self::Int(_), Self::Int(_)) => on_int(self.as_int(), other.as_int()),
            (Self::Text(a), Self::Text(b)) if !a.contains(".") && !b.contains(".") => {
                on_int(self.as_int(), other.as_int())
            }
            (Self::Text(_), _) | (_, Self::Text(_)) => on_float(self.as_float(), other.as_float()),
            (Self::Bool(_), Self::Int(_))
            | (Self::Int(_), Self::Bool(_))
            | (Self::Bool(_), Self::Bool(_)) => on_int(self.as_int(), other.as_int()),
        }
    }
    pub fn is_best_fit_with_float(&self, other: &SValue) -> bool {
        match (self, other) {
            (Self::Float(_), _) | (_, Self::Float(_)) => true,
            (Self::Int(_), Self::Int(_)) => false,
            (Self::Text(a), Self::Text(b)) => a.contains(".") || b.contains("."),
            (Self::Text(text), _) | (_, Self::Text(text)) => text.contains("."),
            (Self::Bool(_), Self::Int(_))
            | (Self::Int(_), Self::Bool(_))
            | (Self::Bool(_), Self::Bool(_)) => false,
        }
    }
}
