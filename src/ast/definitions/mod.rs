mod cmp_node;
mod event_node;
mod expr_node;
mod statement_sequence;
mod stmt_node;

pub use cmp_node::CmpNode;
pub use event_node::EventNode;
pub use expr_node::ExprNode;
pub use statement_sequence::StmtSequenceNode;
pub use stmt_node::StmtNode;

#[derive(Debug, PartialEq, Clone)]
pub enum ExprOrCmpNode {
    Cmp(Box<CmpNode>),
    Expr(Box<ExprNode>),
}
