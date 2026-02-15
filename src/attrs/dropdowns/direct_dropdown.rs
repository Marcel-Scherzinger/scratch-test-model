use crate::attrs::{
    AttributeContentError, ParseJsonBlockAttribute, StringAtArrayPosZero, dropdowns::AnyDropdownOf,
};

/// This is a direct dropdown without a block indirection i. e. extra menu block
#[derive(Debug, PartialEq, Clone, derive_more::Deref)]
pub struct DirectDropdownOf<T>(pub(super) AnyDropdownOf<T>);

impl<T: FromDirectDropdownString> ParseJsonBlockAttribute for DirectDropdownOf<T> {
    fn parse_json_block_attr_without_expecting_shadow<'a>(
        all_target_blocks: crate::_exports::JsonBlocks<'a>,
        attribute_value: &'a serde_json::Value,
    ) -> Result<Self, crate::aux::errors::AttributeContentError<'a>>
    where
        Self: Sized,
    {
        let value = StringAtArrayPosZero::parse_json_block_attr_without_expecting_shadow(
            all_target_blocks,
            attribute_value,
        )?;
        if let Some(selection) = T::from_direct_dropdown_string(&value) {
            Ok(Self(AnyDropdownOf(selection)))
        } else {
            Err(AttributeContentError::InvalidOptionForDropdown {
                invalid: value.take(),
                hint_list: T::VALID_HINT,
            })
        }
    }
}

pub(crate) trait FromDirectDropdownString {
    const VALID_HINT: &'static str;

    fn from_direct_dropdown_string(val: &str) -> Option<Self>
    where
        Self: Sized;
}
