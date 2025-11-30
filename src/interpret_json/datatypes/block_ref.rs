use super::{FormatError, OpcodeNum, get_opcode};
use crate::{ARc, Id};
use derive_more::{Debug, Deref, From};

#[derive(Debug, PartialEq, Deref, From, Eq, PartialOrd, Ord, Hash, Clone)]
#[debug("{_0:?}")]
pub struct RefBlock(Id);

impl RefBlock {
    pub fn id(&self) -> &Id {
        &self.0
    }
    pub fn o_id(&self) -> Id {
        self.0.clone()
    }
    pub(crate) fn parse_from_json(val: &serde_json::Value) -> Result<Self, FormatError> {
        // val is a single string
        if let Some(t) = val.as_str() {
            return Ok(t.into());
        }
        let opcode: OpcodeNum = get_opcode(val)?;

        use crate::constants::*;
        match opcode {
            INPUT_SAME_BLOCK_SHADOW | INPUT_DIFF_BLOCK_SHADOW | INPUT_BLOCK_NO_SHADOW => {
                Self::parse_from_json(&val[1])
            }
            c => Err(FormatError::UnexpectedOpcode(c)),
        }
    }
}

super::_macros::impl_string_from!(RefBlock, ARc<str>);
