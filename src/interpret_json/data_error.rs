#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, thiserror::Error)]
pub enum DataEntityFormatError<T> {
    #[error("missing textual name at index={0}")]
    MissingName(u8),
    #[error("missing textual id at index={0}")]
    MissingId(u8),
    #[error("this variant can't exist so this message should never occur")]
    _Phantom {
        impossible: std::convert::Infallible,
        phantom: std::marker::PhantomData<T>,
    },
}

#[cfg(test)]
mod tests {
    use crate::interpret_json::data_error::DataEntityFormatError;

    #[test]
    fn test_match() {
        match DataEntityFormatError::<()>::MissingName(0) {
            DataEntityFormatError::MissingName(_) => {}
            DataEntityFormatError::MissingId(id) => assert_eq!(0, id),
        }
        match DataEntityFormatError::<()>::MissingId(0) {
            DataEntityFormatError::MissingName(id) => assert_eq!(0, id),
            DataEntityFormatError::MissingId(_) => {}
        }
    }
}
