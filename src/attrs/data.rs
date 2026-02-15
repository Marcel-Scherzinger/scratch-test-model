use crate::{
    Id,
    attrs::{AttributeContentError, ParseJsonBlockAttribute},
    aux::constants,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, derive_getters::Getters)]
pub struct DataId<Kind> {
    kind: std::marker::PhantomData<Kind>,
    name: String,
    id: Id,
}
impl<K> DataId<K> {
    pub fn new(name: String, id: Id) -> Self {
        Self {
            kind: std::marker::PhantomData {},
            name,
            id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VariableKind;
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ListKind;

pub type List = DataId<ListKind>;
pub type Variable = DataId<VariableKind>;

impl<Kind> ParseJsonBlockAttribute for DataId<Kind> {
    fn parse_json_block_attr_without_expecting_shadow<'a>(
        _all_target_blocks: crate::_exports::JsonBlocks<'a>,
        attribute_value: &'a serde_json::Value,
    ) -> Result<Self, crate::aux::errors::AttributeContentError<'a>>
    where
        Self: Sized,
    {
        if let Some(array) = attribute_value.as_array()
            && array.len() == 2
        {
            Self::parse_at_offset(array, 0)
        } else if let Some(array) = attribute_value.as_array()
            && array.len() == 3
            && let Some(constants::VAR_PRIMITIVE | constants::LIST_PRIMITIVE) =
                array.first().and_then(|a| a.as_u64())
        {
            Self::parse_at_offset(array, 1)
        } else {
            Err(AttributeContentError::ExpectedArray {
                value: attribute_value,
                specified_length: Some(2),
            })
        }
    }
}

impl<Kind> DataId<Kind> {
    fn parse_at_offset<'a>(
        array: &'a [serde_json::Value],
        offset: usize,
    ) -> Result<Self, crate::aux::errors::AttributeContentError<'a>> {
        if let Some(name) = array.get(offset).and_then(|a| a.as_str())
            && let Some(id) = array.get(1 + offset).and_then(|a| a.as_str())
        {
            Ok(Self {
                id: id.into(),
                name: name.into(),
                kind: std::marker::PhantomData,
            })
        } else {
            Err(AttributeContentError::Other(
                "expected array with two strings for data (list/var) reference",
            ))
        }
    }
}
