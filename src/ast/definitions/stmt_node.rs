use crate::ast::definitions::ParseAstFromBlocks;
use crate::ast::{CmpNode, ExprOrCmpNode, StmtSequenceNode};
use crate::attr::{DropdownSelection, List, Variable};
use crate::{ARc, Id};

#[derive(Debug, PartialEq, Clone)]
pub enum StmtNode {
    LooksSayforsecs {
        message: ExprOrCmpNode,
        secs: ExprOrCmpNode,
    },
    LooksThink {
        message: ExprOrCmpNode,
    },
    LooksThinkforsecs {
        message: ExprOrCmpNode,
        secs: ExprOrCmpNode,
    },
    LooksSay {
        message: ExprOrCmpNode,
    },

    ControlWait {
        duration: ExprOrCmpNode,
    },
    ControlIf {
        condition: Option<CmpNode>,
        substack: StmtSequenceNode,
    },
    ControlForever {
        substack: StmtSequenceNode,
    },
    ControlStop {
        stop_option: DropdownSelection,
    },
    ControlWaitUntil {
        condition: Option<CmpNode>,
    },
    ControlRepeat {
        times: ExprOrCmpNode,
        substack: StmtSequenceNode,
    },
    ControlIfElse {
        condition: Option<CmpNode>,
        substack: StmtSequenceNode,
        substack2: StmtSequenceNode,
    },
    ControlRepeatuntil {
        condition: Option<CmpNode>,
        substack: StmtSequenceNode,
    },

    DataDeleteoflist {
        list: List,
        index: ExprOrCmpNode,
    },
    DataDeletealloflist {
        list: List,
    },
    DataInsertatlist {
        list: List,
        index: ExprOrCmpNode,
        item: ExprOrCmpNode,
    },
    DataReplaceitemoflist {
        list: List,
        index: ExprOrCmpNode,
        item: ExprOrCmpNode,
    },
    DataAddtolist {
        list: List,
        item: ExprOrCmpNode,
    },
    DataSetvariableto {
        variable: Variable,
        value: ExprOrCmpNode,
    },
    DataChangevariableby {
        variable: Variable,
        value: ExprOrCmpNode,
    },

    SensingAskandwait {
        question: ExprOrCmpNode,
    },

    ProceduresCall {
        // argument_values: HashMap<Id, Expression>,
        proccode: ARc<str>,
        // argumentids: ARc<[ARc<str>]>,
        arguments: ARc<[(Id, Option<ExprOrCmpNode>)]>,
    },
}

impl ParseAstFromBlocks for StmtNode {
    type AstParseErr = StmtNodeAstError;

    fn parse_ast_from_blocks(
        document: &crate::ProjectDoc,
        block_id: &crate::Id,
    ) -> Result<Self, Self::AstParseErr>
    where
        Self: Sized,
    {
        todo!()
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum StmtNodeAstError {}
