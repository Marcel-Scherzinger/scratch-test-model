use std::path::PathBuf;

use crate::{
    Id,
    attrs::data::{ListKind, VariableKind},
    aux::JsonCtx,
};

pub use crate::aux::errors::{
    AttributeContentError, AttributeParseError, BlockJsonStructureError, BlockKindError,
    MandatoryAttrMissingError,
};
pub use crate::scopes::project_doc::NoValidBlockForId;
pub use crate::scopes::target_blocks::BlockReferenceInvalid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, thiserror::Error)]
pub enum TargetDataError<Kind> {
    #[error("expected object {{...}} for data entities of target")]
    ExpectedObject,
    #[error("at least one target data entry (id={0:?}) has unknown structure")]
    AtLeastOneInvalid(Id),
    #[error("SHOULD NEVER OCCUR")]
    Phantom {
        phantom: std::marker::PhantomData<Kind>,
        delete: std::convert::Infallible,
    },
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum BlockError<'a> {
    #[error("missing block attr: {0}")]
    MissingMandatoryAttr(&'static str),

    #[error("block attr '{attr_name}' should have type '{expected_type}'")]
    AttrType {
        attr_name: &'static str,
        expected_type: &'static str,
    },

    #[error("{0}")]
    Kind(BlockKindError<'a>),
}

#[derive(Debug, thiserror::Error)]
pub enum TargetBlocksError<'a> {
    #[error("expected object {{...}} for blocks of target")]
    ExpectedObject,
    #[error("at least one target block (id={id:?}) has unknown structure (block-error={error})")]
    AtLeastOneInvalid {
        id: Id,
        error: JsonCtx<'a, BlockError<'a>>,
    },
}

#[derive(Debug, thiserror::Error, derive_more::From)]
pub enum TargetError<'a> {
    #[error("the attribute \"isStage\" of the target is missing")]
    MissingIsStage,
    #[error("the attribute \"name\" of the target is missing")]
    MissingName,

    /// The saved variable values are provided in an invalid way
    #[error("error in target variables: {0}")]
    TargetVar(JsonCtx<'a, TargetDataError<VariableKind>>),
    /// The saved variable values are provided in an invalid way
    #[error("error in target lists: {0}")]
    TargetLis(JsonCtx<'a, TargetDataError<ListKind>>),
    /// The target blocks (the most important compononet) is invalid
    #[error("error in target blocks: {0}")]
    TargetBlocks(TargetBlocksError<'a>),
}

#[derive(Debug, thiserror::Error, derive_more::From)]
pub enum ModelError<'a> {
    #[error("target error: {0}")]
    Target(JsonCtx<'a, TargetError<'a>>),
    #[error("The document doesn't contain a \"targets\" array: {0}")]
    #[from(skip)]
    NoTargetsArray(&'a serde_json::Value),
}

#[derive(Debug, thiserror::Error, derive_more::From)]
pub enum ParseError<'a> {
    #[error("model: {0}")]
    Model(ModelError<'a>),
    #[error("document: {0}")]
    Doc(DocError),
}

#[derive(Debug, thiserror::Error, derive_more::From)]
pub enum DocError {
    #[error("stream or file was used that doesn't contain a document")]
    NoDocument,
    #[error("io error: {0}")]
    #[from(skip)]
    Io(#[from] Box<dyn std::error::Error>),
    #[error("invalid json document: {0}")]
    #[from(skip)]
    Json(#[from] serde_json::Error),
    #[error("file {0:?} read error: {1}")]
    #[from(skip)]
    FileRead(PathBuf, std::io::Error),
}
