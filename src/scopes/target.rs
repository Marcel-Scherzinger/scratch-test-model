use super::target_data::{TargetLists, TargetVariables};
use crate::aux::WithJsonContextExt as _;
use crate::error::TargetError;
use crate::scopes::TargetBlocks;
use crate::scopes::target_procedures::TargetProcedures;

/// A target is a sprite or the background
///
/// This models a [file format's target object](https://en.scratch-wiki.info/wiki/Scratch_File_Format#Targets)
/// while focussing on the relevant parts for this application.
#[derive(Debug, derive_getters::Getters, PartialEq)]
pub struct Target {
    is_stage: bool,
    name: String,
    variables: TargetVariables,
    lists: TargetLists,
    // broadcasts: (), // not implemented yet
    // comments: (),   // not implemented yet
    blocks: TargetBlocks,
    procedures: TargetProcedures,
}

impl Target {
    pub(crate) fn from_json(value: &serde_json::Value) -> Result<Self, TargetError<'_>> {
        let is_stage = value
            .get("isStage")
            .and_then(|s| s.as_bool())
            .ok_or(TargetError::MissingIsStage)?;
        let name = value
            .get("name")
            .and_then(|s| s.as_str())
            .ok_or(TargetError::MissingName)?
            .into();
        let variables = TargetVariables::from_json.with_ctx(&value["variables"])?;
        let lists = TargetLists::from_json.with_ctx(&value["lists"])?;
        let blocks = TargetBlocks::from_json(&value["blocks"])?;
        let procedures = TargetProcedures::new(&blocks);

        Ok(Self {
            is_stage,
            name,
            variables,
            lists,
            blocks,
            procedures,
        })
    }
}
