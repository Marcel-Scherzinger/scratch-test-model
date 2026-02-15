#[derive(Debug, PartialEq, Clone, derive_more::Deref)]
pub struct AnyDropdownOf<T>(pub(super) T);
