use std::hash::Hash;

use super::{FormatError, OpcodeNum, get_opcode};
use crate::{ARc, Id};
use derive_more::{Debug, Deref};

#[derive(Debug, Deref, PartialOrd, Ord, Clone)]
#[debug("{_0:?}")]
pub struct RefBlock<K>(#[deref] Id, std::marker::PhantomData<K>);

impl<K> RefBlock<K> {
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

super::_macros::impl_string_from!({T} RefBlock<T>, ARc<str>);

impl<K> Eq for RefBlock<K> {}
impl<K> PartialEq for RefBlock<K> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K> Hash for RefBlock<K> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl<K> std::convert::From<ARc<str>> for RefBlock<K> {
    fn from(value: ARc<str>) -> Self {
        Self(value, std::marker::PhantomData)
    }
}
