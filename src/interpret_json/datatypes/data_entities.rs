use super::DataEntityFormatError;
use crate::Id;

use super::{FormatError, get_opcode};

#[derive(Debug, derive_getters::Getters, Clone, PartialEq, Eq)]
pub struct Variable {
    name: String,
    id: Id,
}
#[derive(Debug, Clone, PartialEq, Eq, derive_getters::Getters)]
pub struct List {
    name: String,
    id: Id,
}

impl Variable {
    pub fn new(name: String, id: Id) -> Self {
        Self { name, id }
    }

    pub(super) fn parse_from_array_at_offset(
        arr: &[serde_json::Value],
        offset: u8,
    ) -> Result<Self, DataEntityFormatError<Self>> {
        parse_name_id_from_array(arr, offset).map(|(n, i)| Self::new(n, i))
    }

    pub(crate) fn parse_from_json(val: &serde_json::Value) -> Result<Variable, FormatError> {
        use crate::constants::VAR_PRIMITIVE;
        let (name, id) = get_data_ref::<Variable, VAR_PRIMITIVE>(val)?;
        Ok(Variable { name, id })
    }
}

impl List {
    pub fn new(name: String, id: Id) -> Self {
        Self { name, id }
    }

    pub(super) fn parse_from_array_at_offset(
        arr: &[serde_json::Value],
        offset: u8,
    ) -> Result<Self, DataEntityFormatError<Self>> {
        parse_name_id_from_array(arr, offset).map(|(n, i)| Self::new(n, i))
    }

    pub(crate) fn parse_from_json(val: &serde_json::Value) -> Result<List, FormatError> {
        use crate::constants::LIST_PRIMITIVE;
        let (name, id) = get_data_ref::<List, LIST_PRIMITIVE>(val)?;
        Ok(List::new(name, id))
    }
}

fn get_data_ref<T, const PRIMITIVE_OPCODE: u64>(
    val: &serde_json::Value,
) -> Result<(String, Id), FormatError>
where
    FormatError: From<DataEntityFormatError<T>>,
{
    let arr = val.as_array().ok_or(FormatError::ExpectedArray)?;

    let mut offset: u8 = 0;
    if let Ok(opcode) = get_opcode(val) {
        use crate::constants::*;
        match opcode {
            INPUT_SAME_BLOCK_SHADOW | INPUT_DIFF_BLOCK_SHADOW | INPUT_BLOCK_NO_SHADOW => {
                return get_data_ref::<T, PRIMITIVE_OPCODE>(&val[1]);
            }
            c if c == PRIMITIVE_OPCODE => {
                offset = 1;
            }
            c => return Err(FormatError::UnexpectedOpcode(c)),
        }
    }
    if val.is_null() {
        return Err(FormatError::OpcodeNull);
    }

    Ok(parse_name_id_from_array(arr, offset)?)
}

fn parse_name_id_from_array<T>(
    arr: &[serde_json::Value],
    offset: u8,
) -> Result<(String, Id), DataEntityFormatError<T>> {
    let name = arr[offset as usize] // offset + 0
        .as_str()
        .ok_or(DataEntityFormatError::MissingName(offset))?; // offset + 0
    let id = arr[(offset + 1) as usize]
        .as_str()
        .ok_or(DataEntityFormatError::MissingId(offset + 1))?;
    Ok((name.into(), id.into()))
}
