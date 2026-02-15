use crate::{
    attrs::{List, Variable},
    aux::constants,
    blocks::ExprOrCmpBlockKind,
};
use svalue::SValue;

use crate::attrs::{ParseJsonBlockAttribute, RefBlock};

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Var(Variable),
    Blo(RefBlock<ExprOrCmpBlockKind>),
    Lit(SValue),
    Lis(List),
}

impl ParseJsonBlockAttribute for Expression {
    fn parse_json_block_attr_without_expecting_shadow<'a>(
        all_target_blocks: crate::_exports::JsonBlocks<'a>,
        attribute_value: &'a serde_json::Value,
    ) -> Result<Self, crate::aux::errors::AttributeContentError<'a>>
    where
        Self: Sized,
    {
        if let Some(array) = attribute_value.as_array() {
            if let Some(code) = array.first().and_then(|x| x.as_u64()) {
                if (constants::MATH_NUM_PRIMITIVE == code
                    || constants::INTEGER_NUM_PRIMITIVE == code
                    || constants::POSITIVE_NUM_PRIMITIVE == code
                    || constants::WHOLE_NUM_PRIMITIVE == code
                    || constants::ANGLE_NUM_PRIMITIVE == code)
                    && let Some(value) = array.get(1)
                {
                    if let Some(number) = value.as_number() {
                        return Ok(Expression::Lit(number.clone().into()));
                    } else if let Some(string) = value.as_str() {
                        return Ok(Expression::Lit(SValue::Text(string.into())));
                    }
                }
                if constants::TEXT_PRIMITIVE == code
                    && let Some(arr) = array.get(1)
                    && let Some(value) = arr
                        .as_str()
                        .map(|s| s.to_string())
                        .or_else(|| arr.as_number().map(|s| s.to_string()))
                        .or_else(|| arr.as_bool().map(|s| s.to_string()))
                {
                    return Ok(Expression::Lit(SValue::Text(value.into())));
                }
                if constants::VAR_PRIMITIVE == code {
                    return Variable::parse_json_block_attr_without_expecting_shadow(
                        all_target_blocks,
                        attribute_value,
                    )
                    .map(Expression::Var);
                }
                if constants::LIST_PRIMITIVE == code {
                    return List::parse_json_block_attr_without_expecting_shadow(
                        all_target_blocks,
                        attribute_value,
                    )
                    .map(Expression::Lis);
                }
            }
            Err(crate::aux::errors::AttributeContentError::Other(
                "invalid value for expression",
            ))
        } else if attribute_value.as_str().is_some() {
            RefBlock::parse_json_block_attr_without_expecting_shadow(
                all_target_blocks,
                attribute_value,
            )
            .map(Self::Blo)
        } else {
            Err(super::AttributeContentError::ExpectedArray {
                value: attribute_value,
                specified_length: None,
            })
        }
    }
}
