use super::StmtNode;
use crate::{
    NoValidBlockForId,
    ast::{ParseAstFromBlocks, definitions::StmtNodeAstError},
};

#[derive(Debug, PartialEq, Clone)]
pub struct StmtSequenceNode(Vec<StmtNode>);

impl StmtSequenceNode {
    pub fn empty() -> Self {
        Self(vec![])
    }
}

impl ParseAstFromBlocks for StmtSequenceNode {
    type AstParseErr = StmtSequenceNodeAstError;

    fn parse_ast_from_blocks(
        document: &crate::ProjectDoc,
        block_id: &crate::Id,
    ) -> Result<Self, Self::AstParseErr>
    where
        Self: Sized,
    {
        let mut nodes = vec![];
        let mut id = Some(block_id.clone());

        while let Some(block_id) = id {
            let stmt =
                <StmtNode as ParseAstFromBlocks>::parse_ast_from_blocks(document, &block_id)?;
            nodes.push(stmt);

            let block = document.get_block(&block_id)?;
            id = block.next().clone();
        }
        Ok(StmtSequenceNode(nodes))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StmtSequenceNodeAstError {
    #[error("{0}")]
    BlockNotValid(#[from] NoValidBlockForId),
    #[error("not an event block")]
    NoEvent,
    #[error("stmt-sequence: {0}")]
    Stmt(#[from] StmtNodeAstError),
}
