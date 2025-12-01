use super::StmtNode;

#[derive(Debug, PartialEq, Clone)]
pub struct StmtSequenceNode(Vec<StmtNode>);
