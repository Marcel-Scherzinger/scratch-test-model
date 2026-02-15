use crate::attrs::{AttributeContentError, ParseJsonBlockAttribute};

/// This is a helper datatype that can be parsed as a block attribute.
///
/// It will force that the value of the attribute is a single array,
/// where the first item is a string. This string will be cloned and stored.
/// Other format restrictions are not enforced.
#[derive(Debug, PartialEq, Clone, derive_more::Deref)]
pub struct StringAtArrayPosZero(pub(crate) String);

impl ParseJsonBlockAttribute for StringAtArrayPosZero {
    fn parse_json_block_attr_without_expecting_shadow<'a>(
        _all_target_blocks: crate::_exports::JsonBlocks<'a>,
        attribute_value: &'a serde_json::Value,
    ) -> Result<Self, crate::aux::errors::AttributeContentError<'a>>
    where
        Self: Sized,
    {
        if let Some(array) = attribute_value.as_array()
            && let Some(first) = array.first().and_then(|f| f.as_str())
        {
            return Ok(Self(first.into()));
        }
        Err(AttributeContentError::Other(
            "expected string at first array position",
        ))
    }
}

impl StringAtArrayPosZero {
    pub fn take(self) -> String {
        self.0
    }
}

/// This is a helper datatype that can be parsed as a block attribute.
///
/// It will force that the value of the attribute is a single string
#[derive(Debug, PartialEq, Clone, derive_more::Deref)]
pub struct OnlyString(pub(crate) String);

impl ParseJsonBlockAttribute for OnlyString {
    fn parse_json_block_attr_without_expecting_shadow<'a>(
        _all_target_blocks: crate::_exports::JsonBlocks<'a>,
        attribute_value: &'a serde_json::Value,
    ) -> Result<Self, crate::aux::errors::AttributeContentError<'a>>
    where
        Self: Sized,
    {
        if let Some(value) = attribute_value.as_str() {
            return Ok(Self(value.into()));
        }
        Err(AttributeContentError::Other("expected string as value"))
    }
}

impl OnlyString {
    pub fn take(self) -> String {
        self.0
    }
}
