mod _cmp_using_display;
mod _macros;
mod data_error;
mod datatypes;
mod format_error;
mod number_utils;

pub use _cmp_using_display::CmpUsingDisplay;

pub use data_error::DataEntityFormatError;
pub use datatypes::{
    ArgumentReporterName, DropdownSelection, Expression, List, ProcedureArgumentDef, RefBlock,
    Variable,
};
pub use format_error::FormatError;

use number_utils::get_maybe_number;

pub type OpcodeNum = u64;

pub(crate) fn get_opcode(obj: &serde_json::Value) -> Result<OpcodeNum, FormatError> {
    if obj[0].is_null() {
        return Err(FormatError::OpcodeNull);
    }
    number_utils::get_small_num(obj[0].as_number().ok_or(FormatError::NoNumber)?)
}
