use crate::aux::{AttrLocation, JsonCtx};

#[derive(Debug, thiserror::Error)]
pub enum BlockJsonStructureError {
    #[error("the requested id doesn't exist in the current target")]
    IdNotFound,
    #[error("block is not a json object/mapping, fields not found")]
    BlockIsNoMapping,
    #[error("block mapping doesn't contain mandatory '{0}' option")]
    MissingMandatoryAttr(&'static str),
    #[error("the '{attr}' attribute is of the wrong type, it should be {correct_type}")]
    AttrWrongType {
        attr: &'static str,
        correct_type: &'static str,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum BlockKindError<'a> {
    /// The found opcode is no valid choice for the tried block kind.
    #[error("the found opcode is no valid choice for the tried block kind: {0}")]
    UnknownOpcode(svalue::ARc<str>),
    #[error("json-repr of block: {0}")]
    JsonStructure(#[from] BlockJsonStructureError),
    #[error("missing block attr: {0}")]
    MissingMandatoryAttr(#[from] MandatoryAttrMissingError),
    #[error("parsing attribute {location}.'{key}': {error}")]
    AttrParsing {
        location: AttrLocation,
        key: &'static str,
        error: AttributeParseError<'a>,
    },
}

#[derive(derive_more::Debug, thiserror::Error)]
pub enum AttributeContentError<'a> {
    // Custom-Assoc-Type? / Subblock für alle?
    #[error("{0}")]
    #[debug("{_0:?}")]
    Other(&'static str),

    #[error("'Either' attribute failed twice: first={first_err}, second={second_err}")]
    BothEitherFailed {
        first_err: Box<AttributeContentError<'a>>,
        second_err: Box<AttributeContentError<'a>>,
    },

    #[error("expected an array (fixed-len={specified_length:?}): {value}")]
    ExpectedArray {
        value: &'a serde_json::Value,
        specified_length: Option<usize>,
    },

    #[error("value {invalid:?} is invalid for dropdown, valid examples/hints: {hint_list}")]
    InvalidOptionForDropdown {
        invalid: String,
        hint_list: &'static str,
    },

    #[error("this dropdown is expected to have a string id as reference to it's menu, not: {0}")]
    ExternalDropdownMenuReferenceNotString(&'a serde_json::Value),

    #[error("there was an error parsing an indirection subblock: {0}")]
    Subblock(Box<JsonCtx<'a, BlockKindError<'a>>>),
}

#[derive(Debug, thiserror::Error)]
pub enum AttributeParseError<'a> {
    /// The attributes only value is a singleton array indicating that it contains shadow
    /// information, but the array has no real data
    ///
    /// *This error shouldn't be produced by normal attribute parsing implementations*
    #[error("attribute is not providing a real value, only shadow indication")]
    ShadowWithoutValue,
    /// The attribute has to nested shadow levels,
    /// when the first is traversed (to the block hiding the possible shadow)
    /// another shadow information was found. This is invalid
    ///
    /// *This error shouldn't be produced by normal attribute parsing implementations*
    #[error("attribute contains two levels of shadow information, only one allowed")]
    DuplicateShadow,
    #[error("content, shadowing={shadow_resolved}: {error}")]
    Content {
        shadow_resolved: bool,
        error: AttributeContentError<'a>,
    },
}

#[derive(Debug, thiserror::Error)]
#[error(
    "the mandatory block attribute '{key}' is missing from the expected property set '{location}'"
)]
pub struct MandatoryAttrMissingError {
    pub(super) location: AttrLocation,
    pub(super) key: &'static str,
}
