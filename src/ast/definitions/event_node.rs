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
