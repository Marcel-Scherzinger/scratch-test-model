mod cmp_node;
mod event_node;
mod expr_node;
mod project_node;
mod statement_sequence;
mod stmt_node;
mod track_node;

pub use cmp_node::CmpNode;
pub use event_node::EventNode;
pub use expr_node::ExprNode;
pub use statement_sequence::StmtSequenceNode;
pub use stmt_node::StmtNode;
pub use stmt_node::StmtNodeAstError;
pub use track_node::TrackNode;

#[derive(Debug, PartialEq, Clone)]
pub enum ExprOrCmpNode {
    Cmp(Box<CmpNode>),
    Expr(Box<ExprNode>),
}

#[derive(Debug, PartialEq, Clone, thiserror::Error)]
#[error("[block={id:?}] {err}")]
pub struct AstError<E> {
    id: crate::Id,
    err: E,
}
impl<E> AstError<E> {
    pub fn new(id: crate::Id, err: E) -> Self {
        Self { id, err }
    }
}

pub trait ParseAstFromBlocks {
    type AstParseErr;

    fn parse_ast_from_blocks(
        document: &crate::ProjectDoc,
        block_id: &crate::Id,
    ) -> Result<Self, Self::AstParseErr>
    where
        Self: Sized;

    fn ctx_parse_ast_from_blocks(
        document: &crate::ProjectDoc,
        block_id: &crate::Id,
    ) -> Result<Self, AstError<Self::AstParseErr>>
    where
        Self: Sized,
    {
        Self::parse_ast_from_blocks(document, block_id)
            .map_err(|err| AstError::new(block_id.clone(), err))
    }
}
