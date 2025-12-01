use super::ExprOrCmpNode;
use crate::attr::{ArgumentReporterName, DropdownSelection, List, Variable};

#[derive(Debug, PartialEq, Clone)]
pub enum ExprNode {
    ArgumentReporterStringNumber {
        value: ArgumentReporterName,
    },
    ArgumentReporterBoolean {
        value: ArgumentReporterName,
    },

    OperatorLength {
        string: ExprOrCmpNode,
    },
    OperatorJoin {
        string1: ExprOrCmpNode,
        string2: ExprOrCmpNode,
    },
    OperatorAdd {
        num1: ExprOrCmpNode,
        num2: ExprOrCmpNode,
    },
    OperatorMultiply {
        num1: ExprOrCmpNode,
        num2: ExprOrCmpNode,
    },
    OperatorMod {
        num1: ExprOrCmpNode,
        num2: ExprOrCmpNode,
    },
    OperatorSubtract {
        num1: ExprOrCmpNode,
        num2: ExprOrCmpNode,
    },
    OperatorDivide {
        num1: ExprOrCmpNode,
        num2: ExprOrCmpNode,
    },
    OperatorMathop {
        operator: DropdownSelection,
        num: ExprOrCmpNode,
    },
    OperatorLetterOf {
        letter: ExprOrCmpNode,
        string: ExprOrCmpNode,
    },
    OperatorRound {
        num: ExprOrCmpNode,
    },
    OperatorRandom {
        from: ExprOrCmpNode,
        to: ExprOrCmpNode,
    },

    SensingAnswer,

    DataItemnumoflist {
        list: List,
        item: ExprOrCmpNode,
    },
    DataItemoflist {
        list: List,
        index: ExprOrCmpNode,
    },
    DataLengthoflist {
        list: List,
    },

    // reading a list is not a real block, use this pseudo block
    RDataList {
        list: List,
    },
    // reading a variable is not a real block, use this pseudo block
    RDataVar {
        variable: Variable,
    },
}
