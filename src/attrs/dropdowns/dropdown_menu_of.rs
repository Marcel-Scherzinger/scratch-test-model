use super::AnyDropdownOf;
use crate::{
    _exports::ParseJsonBlock,
    attrs::{AttributeContentError as A, ParseJsonBlockAttribute},
};

/// This defines a strategy how to parse a dropdown menu block
/// that should hold values of type `For`.
///
/// This trait is generic so that a strategy can support multiple option sets.
/// It's meant as a separate strategy type so that one and the same value set
/// can be parsed in multiple ways.
pub trait ExtraMenuDropdownParseStrat<For> {
    const VALID_HINT: &'static str;
    type MenuBlock: ParseJsonBlock;

    /// Converts an external menu block to the requested value set type `For`.
    ///
    /// If it is an invalid value the function shall return a [`String`]
    /// representing the found value that is considered invalid
    fn from_menu_block(block: Self::MenuBlock) -> Result<For, String>
    where
        Self: Sized;
}

// represents a dropdown where an extra menu block is introduced as indirection
#[derive(Debug, PartialEq, Clone, derive_more::Deref)]
pub struct DropdownMenuOf<T, S = T> {
    #[deref]
    data: AnyDropdownOf<T>,
    marker: std::marker::PhantomData<S>,
}

impl<T, S: ExtraMenuDropdownParseStrat<T>> ParseJsonBlockAttribute for DropdownMenuOf<T, S> {
    fn parse_json_block_attr_without_expecting_shadow<'a>(
        all_target_blocks: crate::_exports::JsonBlocks<'a>,
        attribute_value: &'a serde_json::Value,
    ) -> Result<Self, crate::aux::errors::AttributeContentError<'a>>
    where
        Self: Sized,
    {
        if let Some(block_reference) = attribute_value.as_str() {
            let block = S::MenuBlock::ctx_parse_json_block(all_target_blocks, block_reference)
                .map_err(|b| A::Subblock(b.into()))?;
            let selected = S::from_menu_block(block);

            let value = selected.map_err(|invalid| A::InvalidOptionForDropdown {
                invalid,
                hint_list: S::VALID_HINT,
            })?;
            let any_dropdown = AnyDropdownOf(value);
            Ok(Self {
                data: any_dropdown,
                marker: std::marker::PhantomData,
            })
        } else {
            Err(A::ExternalDropdownMenuReferenceNotString(attribute_value))
        }
    }
}
