use serde_json::{Map, Value};

use crate::aux::{
    AttrLocation, JsonCtx,
    errors::{BlockJsonStructureError, BlockKindError, MandatoryAttrMissingError},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct JsonBlocks<'a>(&'a serde_json::Value);
impl<'a> JsonBlocks<'a> {
    pub(crate) fn new(v: &'a serde_json::Value) -> Self {
        Self(v)
    }
    #[allow(unused)]
    pub(crate) fn get_block(&self, id: &str) -> Option<&serde_json::Value> {
        self.0.get(id)
    }
}

/// Convenience wrapper that contains the `inputs` or `fields`
/// mapping of a block. It provides a method to access named
/// attributes, producing a failure if it doesn't exist.
pub struct BlockProperties<'a>(AttrLocation, &'a Map<String, Value>);

impl<'a> BlockProperties<'a> {
    /// returns an attribute with a statically known key, producing
    /// an error if the attribute doesn't exist
    pub fn force_static_attr<'b>(
        &'b self,
        key: &'static str,
    ) -> Result<&'a Value, MandatoryAttrMissingError> {
        self.1.get(key).ok_or(MandatoryAttrMissingError {
            location: self.0,
            key,
        })
    }
    pub fn location(&self) -> &AttrLocation {
        &self.0
    }
    pub fn data(&self) -> &'a Map<String, Value> {
        self.1
    }
}

/// Implemented by block kinds for parsing blocks from their json structures
///
/// [`ParseJsonBlock::parse_json_block`] needs to be implemented while
/// [`ParseJsonBlock::ctx_parse_json_block`] adds contextual information
/// and wraps a call to the other method.
pub trait ParseJsonBlock {
    fn parse_json_block<'a, 'b>(
        all_target_blocks: JsonBlocks<'a>,
        opcode: &str,
        inputs: &'b BlockProperties<'a>,
        fields: &'b BlockProperties<'a>,
        mutation: Option<&'b BlockProperties<'a>>,
    ) -> Result<Self, BlockKindError<'a>>
    where
        Self: Sized;

    fn ctx_parse_json_block<'a>(
        all_target_blocks: JsonBlocks<'a>,
        id: &str,
    ) -> Result<Self, JsonCtx<'a, BlockKindError<'a>>>
    where
        Self: Sized,
    {
        // block_data *should* be: { "opcode": "...", "inputs": {...}, "fields": {...}, ... }

        // ensure block exists
        let block_data = all_target_blocks.0.get(id).ok_or_else(|| JsonCtx {
            inner_error: BlockJsonStructureError::IdNotFound.into(),
            json_ctx: all_target_blocks.0,
        })?;
        // ensure block is mapping
        let block_data_map = block_data.as_object().ok_or_else(|| JsonCtx {
            inner_error: BlockJsonStructureError::BlockIsNoMapping.into(),
            json_ctx: block_data,
        })?;

        // ensure mapping contains "opcode" key with string value
        let opcode_val = block_data_map.get("opcode").ok_or_else(|| JsonCtx {
            inner_error: BlockJsonStructureError::MissingMandatoryAttr("opcode").into(),
            json_ctx: block_data,
        })?;
        let opcode = opcode_val.as_str().ok_or_else(|| JsonCtx {
            inner_error: BlockJsonStructureError::AttrWrongType {
                attr: "opcode",
                correct_type: "string",
            }
            .into(),
            json_ctx: block_data,
        })?;

        let access_object = |key: &'static str| {
            block_data_map
                .get(key)
                .ok_or(BlockJsonStructureError::MissingMandatoryAttr(key).into())
                .and_then(|hopefully_object| {
                    hopefully_object.as_object().ok_or(
                        BlockJsonStructureError::AttrWrongType {
                            attr: key,
                            correct_type: "object",
                        }
                        .into(),
                    )
                })
                .map_err(|err| JsonCtx {
                    inner_error: err,
                    json_ctx: block_data,
                })
        };

        // ensure inputs exists and is a mapping
        let inputs = BlockProperties(AttrLocation::Inputs, access_object("inputs")?);
        // ensure fields exists and is a mapping
        let fields = BlockProperties(AttrLocation::Fields, access_object("fields")?);

        let mutation = block_data_map
            .get("mutation")
            .map(|val| {
                val.as_object()
                    .ok_or(
                        BlockJsonStructureError::AttrWrongType {
                            attr: "mutation",
                            correct_type: "object",
                        }
                        .into(),
                    )
                    .map_err(|err| JsonCtx {
                        inner_error: err,
                        json_ctx: block_data,
                    })
            })
            .transpose()?
            .map(|data| BlockProperties(AttrLocation::Mutation, data));

        let parsed = Self::parse_json_block(
            all_target_blocks,
            opcode,
            &inputs,
            &fields,
            mutation.as_ref(),
        );
        parsed.map_err(|err| JsonCtx {
            inner_error: err,
            json_ctx: block_data,
        })
    }
}
