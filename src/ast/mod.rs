mod _macros;
mod definitions;

use crate::blocks;
use _macros::associate_ast_types;
use definitions::ParseAstFromBlocks;

use definitions::{CmpNode, ExprNode, ExprOrCmpNode, StmtNode, StmtSequenceNode};

pub trait IsAssociatedWithAstDataType {
    type AstDataType;
}

associate_ast_types!(
    Vec<blocks::StmtBlockKind> => StmtSequenceNode,
    crate::attr::Expression => ExprOrCmpNode,
    blocks::ExprBlockKind => ExprNode,
    blocks::CmpBlockKind => CmpNode,
);
