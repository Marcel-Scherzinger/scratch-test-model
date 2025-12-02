use crate::NoValidBlockForId;
use crate::ast::ParseAstFromBlocks;
use crate::ast::definitions::AstError;
use crate::ast::definitions::statement_sequence::StmtSequenceNodeAstError;
use crate::{ast::definitions::StmtSequenceNode, attr::DropdownSelection};

#[derive(Debug, PartialEq, Clone)]
pub enum EventNode {
    EventWhenflagclicked {
        stmts: StmtSequenceNode,
    },
    EventWhenkeypressed {
        key_option: DropdownSelection,
        stmts: StmtSequenceNode,
    },
}

impl ParseAstFromBlocks for EventNode {
    type AstParseErr = EventNodeAstError;

    fn parse_ast_from_blocks(
        document: &crate::ProjectDoc,
        block_id: &crate::Id,
    ) -> Result<Self, Self::AstParseErr>
    where
        Self: Sized,
    {
        let block = document.get_block(block_id)?;
        use crate::BlockKind as B;
        use crate::EventBlockKind as E;
        match block.inner() {
            B::Event(event) => {
                let stmts = if let Some(next_id) = block.next() {
                    <StmtSequenceNode as ParseAstFromBlocks>::ctx_parse_ast_from_blocks(
                        document, next_id,
                    )?
                } else {
                    StmtSequenceNode::empty()
                };

                Ok(match event {
                    E::EventWhenkeypressed { key_option } => Self::EventWhenkeypressed {
                        key_option: key_option.clone(),
                        stmts,
                    },
                    E::EventWhenflagclicked => Self::EventWhenflagclicked { stmts },
                })
            }
            _ => Err(EventNodeAstError::NoEvent),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EventNodeAstError {
    #[error("{0}")]
    BlockNotValid(#[from] NoValidBlockForId),
    #[error("not an event block")]
    NoEvent,
    #[error("stmt-sequence: {0}")]
    StmtSequence(#[from] AstError<StmtSequenceNodeAstError>),
}
