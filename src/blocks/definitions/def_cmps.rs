use crate::attr::{ArgumentReporterName, Expression, List, RefBlock};

super::define_blocks! {
    #[derive(Debug, PartialEq, Clone)]
    pub enum CmpBlockKind (CmpBlockKindUnit):

    "argument_reporter_boolean" => ArgumentReporterBoolean {
        (field) value: ArgumentReporterName,
    },

    "operator_and" => OperatorAnd {
        operand1: RefBlock<CmpBlockKind>,
        operand2: RefBlock<CmpBlockKind>,
    },
    "operator_or" => OperatorOr {
        operand1: RefBlock<CmpBlockKind>,
        operand2: RefBlock<CmpBlockKind>,
    },
    "operator_equals" => OperatorEquals {
        operand1: Expression,
        operand2: Expression,
    },
    "operator_gt" => OperatorGt {
        operand1: Expression,
        operand2: Expression,
    },
    "operator_lt" => OperatorLt {
        operand1: Expression,
        operand2: Expression,
    },
    "operator_contains" => OperatorContains {
        string1: Expression,
        string2: Expression,
    },
    "operator_not" => OperatorNot {
        /// reference to boolean expression that should be negated
        operand: RefBlock<CmpBlockKind>,
    },

    "data_listcontainsitem" => DataListcontainsitem {
        (field) list: List,
        item: Expression,
    },
}
