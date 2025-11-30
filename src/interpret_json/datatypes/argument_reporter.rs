use super::{FormatError, OpcodeNum, get_opcode};
use crate::ARc;
use derive_more::{Debug, Deref, From};

#[derive(Debug, PartialEq, Deref, From, Clone)]
#[debug("{_0:?}")]
pub struct ArgumentReporterName(ARc<str>);

super::_macros::impl_string_from!(ArgumentReporterName, ARc<str>);

impl ArgumentReporterName {
    pub(crate) fn parse_from_json(
        val: &serde_json::Value,
    ) -> Result<ArgumentReporterName, FormatError> {
        // val is a single string
        if let Some(t) = val.as_str() {
            return Ok(t.into());
        } else if let Some(arr) = val.as_array()
            && let Some(t) = arr[0].as_str()
        {
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
