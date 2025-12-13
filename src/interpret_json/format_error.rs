use super::CmpUsingDisplay;
use crate::{
    attr::{List, Variable},
    interpret_json::{OpcodeNum, data_error::DataEntityFormatError},
};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum FormatError {
    #[error("unable to parse number")]
    NoNumber,
    #[error("opcode={opcode} is not allowed to have {value:?} as value")]
    UnexpectedNumberKind {
        opcode: OpcodeNum,
        value: serde_json::Value,
    },
    #[error("number not in scope of expected type")]
    NumberOutOfRange,
    #[error("read opcode was null")]
    OpcodeNull,
    #[error("found opcode={0} is invalid at this point")]
    UnexpectedOpcode(OpcodeNum),

    #[error("variable error: {0}")]
    Variable(#[from] DataEntityFormatError<Variable>),
    #[error("list error: {0}")]
    List(#[from] DataEntityFormatError<List>),

    #[error("missing text primitive at index={0}")]
    MissingTextPrim(u8),
    #[error("expected array")]
    ExpectedArray,

    #[error("expected serialised json: {0}")]
    ExpectedSerialisedJson(CmpUsingDisplay<serde_json::Error>),
    #[error("expected arrays of equal length")]
    ArraysMustHaveSameLength,
}
