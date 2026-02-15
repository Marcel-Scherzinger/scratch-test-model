use crate::{attrs::AttributeContentError as A, aux::constants};

#[derive(Debug, PartialEq, Clone)]
pub struct BroadcastId {
    human_name: svalue::ARc<str>,
    id: crate::Id,
}

const BROADCAST_HUMAN_NAME: A<'static> = A::Other("'human name' not present or not string");
const BROADCAST_ID: A<'static> = A::Other("'id' not present or not string");
const BROADCAST_ARRAY: A<'static> = A::Other("value is not an array");

impl super::ParseJsonBlockAttribute for BroadcastId {
    fn parse_json_block_attr_without_expecting_shadow<'a>(
        _all_target_blocks: crate::_exports::JsonBlocks<'a>,
        attribute_value: &'a serde_json::Value,
    ) -> Result<Self, super::AttributeContentError<'a>>
    where
        Self: Sized,
    {
        if let Some(array) = attribute_value.as_array() {
            let offset =
                if array.first().and_then(|o| o.as_u64()) == Some(constants::BROADCAST_PRIMITIVE) {
                    1
                } else {
                    0
                };

            let human_name = array
                .get(offset)
                .and_then(|s| s.as_str())
                .ok_or(BROADCAST_HUMAN_NAME)?
                .into();
            let id = array
                .get(offset + 1)
                .and_then(|s| s.as_str())
                .ok_or(BROADCAST_ID)?
                .into();
            return Ok(Self { human_name, id });
        }
        Err(BROADCAST_ARRAY)
    }
}
