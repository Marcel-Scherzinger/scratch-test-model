use crate::{
    _exports::{BlockKindError, BlockProperties, JsonBlocks},
    attrs::AttributeParseError,
    aux::{AttrLocation, errors::AttributeContentError},
};

pub trait ParseJsonBlockAttribute {
    #[allow(unused)]
    fn mandatory_attr_missing_fallback(
        all_target_blocks: JsonBlocks<'_>,
        location: AttrLocation,
        key: &'static str,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }

    fn parse_json_block_attr_with_possible_shadow<'a>(
        all_target_blocks: JsonBlocks<'a>,
        attribute_value: &'a serde_json::Value,
    ) -> Result<Self, AttributeParseError<'a>>
    where
        Self: Sized,
    {
        use crate::aux::constants as C;

        // if there is shadow information, the value will be an array [NUM_CODE, Value1 (, Value2)?]
        if let Some(array) = attribute_value.as_array() {
            let maybe_numeric_code = array.first().and_then(|v| v.as_u64());

            let is_shadow_num_code = |num| {
                num == C::INPUT_SAME_BLOCK_SHADOW
                    || num == C::INPUT_DIFF_BLOCK_SHADOW
                    || num == C::INPUT_BLOCK_NO_SHADOW
            };

            if maybe_numeric_code.is_some_and(is_shadow_num_code) {
                // If there is a shadow, the current information (the one hiding it)
                // will be on array position 1. A possible sadow is at index 2.
                // The shadow isn't interesing so it can be ignored. Only index 1 is processed.

                if array
                    .get(1)
                    .and_then(|v| v.as_u64())
                    .is_some_and(is_shadow_num_code)
                {
                    // If the main information itself is a shadow declaration,
                    // the format is invalid
                    return Err(AttributeParseError::DuplicateShadow);
                }

                // add information if a shadow was present to the possible error value
                return Self::parse_json_block_attr_without_expecting_shadow(
                    all_target_blocks,
                    array
                        .get(1)
                        .ok_or(AttributeParseError::ShadowWithoutValue)?,
                )
                .map_err(|error| AttributeParseError::Content {
                    shadow_resolved: true,
                    error,
                });
            }
        }

        // if this line is reached, no shadow information was found
        Self::parse_json_block_attr_without_expecting_shadow(all_target_blocks, attribute_value)
            .map_err(|error| AttributeParseError::Content {
                shadow_resolved: false,
                error,
            })
    }

    fn parse_json_block_attr_without_expecting_shadow<'a>(
        all_target_blocks: JsonBlocks<'a>,
        attribute_value: &'a serde_json::Value,
    ) -> Result<Self, AttributeContentError<'a>>
    where
        Self: Sized;
}

pub(crate) fn helper_attr_access<'a, T>(
    all_target_blocks: JsonBlocks<'a>,
    properties: &BlockProperties<'a>,
    key: &'static str,
) -> Result<T, BlockKindError<'a>>
where
    T: ParseJsonBlockAttribute,
{
    let location = *(properties.location());
    let value = properties.force_static_attr(key);
    match value {
        Err(err) => {
            let fallback =
                T::mandatory_attr_missing_fallback(all_target_blocks, *properties.location(), key);
            fallback.ok_or_else(|| err.into())
        }
        Ok(attribute_value) => {
            let parsed =
                T::parse_json_block_attr_with_possible_shadow(all_target_blocks, attribute_value);
            let val = parsed.map_err(move |error| BlockKindError::AttrParsing {
                location,
                key,
                error,
            })?;
            Ok(val)
        }
    }
}
