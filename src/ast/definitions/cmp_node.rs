use super::ExprOrCmpNode;
use crate::attr::{ArgumentReporterName, List};

#[derive(Debug, PartialEq, Clone)]
pub enum CmpNode {
    ArgumentReporterBoolean {
        value: ArgumentReporterName,
    },

    OperatorAnd {
        operand1: Box<CmpNode>,
        operand2: Box<CmpNode>,
    },
    OperatorOr {
        operand1: Box<CmpNode>,
        operand2: Box<CmpNode>,
    },
    OperatorEquals {
        operand1: ExprOrCmpNode,
        operand2: ExprOrCmpNode,
    },
    OperatorGt {
        operand1: ExprOrCmpNode,
        operand2: ExprOrCmpNode,
    },
    OperatorLt {
        operand1: ExprOrCmpNode,
        operand2: ExprOrCmpNode,
    },
    OperatorContains {
        string1: ExprOrCmpNode,
        string2: ExprOrCmpNode,
    },
    OperatorNot {
        /// reference to boolean expression that should be negated
        operand: Box<CmpNode>,
    },

    DataListcontainsitem {
        list: List,
        item: ExprOrCmpNode,
    },
}
