mod broadcast_id;
mod color;
pub(crate) mod data;
pub mod dropdowns;
mod expression;
mod string_helper;

pub use color::Color;

pub use data::{List, Variable};

pub use dropdowns::{AnyDropdownOf, DirectDropdownOf, DropdownMenuOf};

pub use crate::scopes::block_wrapper::Position;

use either::Either;
pub use expression::Expression;
pub(crate) use string_helper::StringAtArrayPosZero;

pub use broadcast_id::BroadcastId;

use crate::attrs::string_helper::OnlyString;
pub use crate::aux::errors::{AttributeContentError, AttributeParseError};
pub(crate) use crate::aux::parse_attr::ParseJsonBlockAttribute;

pub use dropdowns::{ExpressionRef, RoundDirectDropdownOf, RoundDropdownMenuOf};

#[derive(Debug, PartialEq, Clone, derive_more::Deref)]
pub struct ArgumentReporterName(StringAtArrayPosZero);

impl ParseJsonBlockAttribute for ArgumentReporterName {
    fn parse_json_block_attr_without_expecting_shadow<'a>(
        all_target_blocks: crate::_exports::JsonBlocks<'a>,
        attribute_value: &'a serde_json::Value,
    ) -> Result<Self, crate::aux::errors::AttributeContentError<'a>>
    where
        Self: Sized,
    {
        StringAtArrayPosZero::parse_json_block_attr_without_expecting_shadow(
            all_target_blocks,
            attribute_value,
        )
        .map(Self)
    }
}

impl<FirstTry, SecondTry> ParseJsonBlockAttribute for Either<FirstTry, SecondTry>
where
    FirstTry: ParseJsonBlockAttribute,
    SecondTry: ParseJsonBlockAttribute,
{
    fn parse_json_block_attr_without_expecting_shadow<'a>(
        all_target_blocks: crate::_exports::JsonBlocks<'a>,
        attribute_value: &'a serde_json::Value,
    ) -> Result<Self, crate::aux::errors::AttributeContentError<'a>>
    where
        Self: Sized,
    {
        let first = FirstTry::parse_json_block_attr_without_expecting_shadow(
            all_target_blocks,
            attribute_value,
        )
        .map(Self::Left);
        match first {
            Ok(f) => Ok(f),
            Err(first_err) => {
                let second = SecondTry::parse_json_block_attr_without_expecting_shadow(
                    all_target_blocks,
                    attribute_value,
                )
                .map(Self::Right);
                second.map_err(|second_err| AttributeContentError::BothEitherFailed {
                    first_err: Box::new(first_err),
                    second_err: Box::new(second_err),
                })
            }
        }
    }
}

#[derive(derive_more::Debug, PartialEq, Clone)]
#[debug("RefBlock({id:?})")]
pub struct RefBlock<T> {
    id: crate::Id,
    phantom: std::marker::PhantomData<T>,
}
impl<T> RefBlock<T> {
    pub fn o_id(&self) -> crate::Id {
        self.id.clone()
    }
    pub fn id(&self) -> &crate::Id {
        &self.id
    }
}

impl<T> ParseJsonBlockAttribute for RefBlock<T> {
    fn parse_json_block_attr_without_expecting_shadow<'a>(
        all_target_blocks: crate::_exports::JsonBlocks<'a>,
        attribute_value: &'a serde_json::Value,
    ) -> Result<Self, crate::aux::errors::AttributeContentError<'a>>
    where
        Self: Sized,
    {
        let value = OnlyString::parse_json_block_attr_without_expecting_shadow(
            all_target_blocks,
            attribute_value,
        );
        value.map(|value: OnlyString| RefBlock {
            id: value.take().into(),
            phantom: std::marker::PhantomData,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct ProcedureArgumentId(pub(crate) crate::Id);

impl ProcedureArgumentId {
    // not a block id but similar pattern
    pub fn id(&self) -> &svalue::ARc<str> {
        &self.0
    }
}

impl ParseJsonBlockAttribute for ProcedureArgumentId {
    fn parse_json_block_attr_without_expecting_shadow<'a>(
        all_target_blocks: crate::_exports::JsonBlocks<'a>,
        attribute_value: &'a serde_json::Value,
    ) -> Result<Self, crate::aux::errors::AttributeContentError<'a>>
    where
        Self: Sized,
    {
        let value = OnlyString::parse_json_block_attr_without_expecting_shadow(
            all_target_blocks,
            attribute_value,
        );
        value.map(|value: OnlyString| ProcedureArgumentId(value.take().into()))
    }
}

pub type KeyboardKey = svalue::ARc<str>;

/// A field that has type `Option<T>` can be missing from the attribute
/// set. In this case, the parsed value will be `None`.
/// Otherwise `T`s parser will continue.
///
/// This also includes that the value could be the JSON `null`.
impl<T: ParseJsonBlockAttribute> ParseJsonBlockAttribute for Option<T> {
    fn mandatory_attr_missing_fallback(
        _all_target_blocks: crate::aux::JsonBlocks<'_>,
        _location: crate::aux::AttrLocation,
        _key: &'static str,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        Some(None)
    }

    fn parse_json_block_attr_without_expecting_shadow<'a>(
        all_target_blocks: crate::_exports::JsonBlocks<'a>,
        attribute_value: &'a serde_json::Value,
    ) -> Result<Self, crate::aux::errors::AttributeContentError<'a>>
    where
        Self: Sized,
    {
        if attribute_value.is_null() {
            return Ok(None);
        }
        T::parse_json_block_attr_without_expecting_shadow(all_target_blocks, attribute_value)
            .map(Some)
    }
}
