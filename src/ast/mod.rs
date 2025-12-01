mod _macros;
mod definitions;

use _macros::associate_ast_types;

use definitions::{CmpNode, ExprNode, ExprOrCmpNode, StmtNode, StmtSequenceNode};

pub trait IsAssociatedWithAstDataType {
    type AstDataType;
}
