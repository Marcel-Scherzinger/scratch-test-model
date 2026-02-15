use crate::{
    attrs::{AttributeContentError, ParseJsonBlockAttribute},
    aux::constants,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Color {
    hex: svalue::ARc<str>,
}

impl ParseJsonBlockAttribute for Color {
    fn parse_json_block_attr_without_expecting_shadow<'a>(
        _all_target_blocks: crate::_exports::JsonBlocks<'a>,
        attribute_value: &'a serde_json::Value,
    ) -> Result<Self, crate::aux::errors::AttributeContentError<'a>>
    where
        Self: Sized,
    {
        if let Some(array) = attribute_value.as_array() {
            if let Some(code) = array.first().and_then(|x| x.as_u64()) {
                if constants::COLOR_PICKER_PRIMITIVE == code {
                    if let Some(hex) = array.get(1).and_then(|v| v.as_str()) {
                        Ok(Self { hex: hex.into() })
                    } else {
                        Err(AttributeContentError::Other(
                            "value is invalid hex code for color picker",
                        ))
                    }
                } else {
                    Err(AttributeContentError::Other(
                        "numeric code invalid for color picker",
                    ))
                }
            } else {
                Err(AttributeContentError::Other(
                    "expected numeric code for color picker",
                ))
            }
        } else {
            Err(AttributeContentError::ExpectedArray {
                value: attribute_value,
                specified_length: Some(2),
            })
        }
    }
}
