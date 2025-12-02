use crate::NoValidBlockForId;
use crate::ast::ParseAstFromBlocks;
use crate::ast::definitions::event_node::EventNodeAstError;
use crate::ast::definitions::statement_sequence::StmtSequenceNodeAstError;
use crate::ast::definitions::{EventNode, StmtSequenceNode};

#[derive(Debug, PartialEq, Clone)]
pub enum TrackNode {
    Headless(StmtSequenceNode),
    WithHead(EventNode),
}

#[derive(Debug, thiserror::Error)]
pub enum TrackNodeAstError {
    #[error("{0}")]
    BlockNotValid(#[from] NoValidBlockForId),
    #[error("neither statement nor event block")]
    NeitherStmtNorEvent,
    #[error("event: {0}")]
    Event(#[from] EventNodeAstError),
    #[error("stmt-sequence: {0}")]
    StmtSeq(#[from] StmtSequenceNodeAstError),
}

impl ParseAstFromBlocks for TrackNode {
    type AstParseErr = TrackNodeAstError;

    fn parse_ast_from_blocks(
        document: &crate::ProjectDoc,
        block_id: &crate::Id,
    ) -> Result<Self, Self::AstParseErr>
    where
        Self: Sized,
    {
        let block = document.get_block(block_id)?;
        use crate::BlockKind as B;
        // TODO: maybe move Unsupported/NoOp blocks into corresponding
        // Stmt/Expr/Cmp block kinds so that matching is easier
        match block.inner() {
            B::Expr(_)
            | B::Cmp(_)
            | B::ProceduresPrototype { .. }
            | B::ProceduresDefinition { .. }
            | B::Noop(_)
            | B::Unsup(_) => Err(TrackNodeAstError::NeitherStmtNorEvent),
            B::Event(_) => Ok(TrackNode::WithHead(
                <EventNode as ParseAstFromBlocks>::parse_ast_from_blocks(document, block_id)?,
            )),
            B::Stmt(_) => Ok(TrackNode::Headless(
                <StmtSequenceNode as ParseAstFromBlocks>::parse_ast_from_blocks(
                    document, block_id,
                )?,
            )),
        }
    }
}
