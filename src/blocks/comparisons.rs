use scratch_test_model_proc::Block;

use crate::attrs::{
    ArgumentReporterName, Color, DirectDropdownOf, DropdownMenuOf, Expression, List, RefBlock,
    dropdowns::{SensingKeyoptions, SensingTouchingobject},
};

#[derive(Debug, PartialEq, Block, Clone)]
#[block(default_location = inputs)]
pub enum CmpBlockKind {
    ArgumentReporterBoolean {
        #[block(location = fields)]
        value: ArgumentReporterName,
    },

    OperatorAnd {
        operand1: Option<RefBlock<CmpBlockKind>>,
        operand2: Option<RefBlock<CmpBlockKind>>,
    },
    OperatorOr {
        operand1: Option<RefBlock<CmpBlockKind>>,
        operand2: Option<RefBlock<CmpBlockKind>>,
    },
    OperatorEquals {
        operand1: Expression,
        operand2: Expression,
    },
    OperatorGt {
        operand1: Expression,
        operand2: Expression,
    },
    OperatorLt {
        operand1: Expression,
        operand2: Expression,
    },
    OperatorContains {
        string1: Expression,
        string2: Expression,
    },
    OperatorNot {
        /// reference to boolean expression that should be negated
        operand: Option<RefBlock<CmpBlockKind>>,
    },

    DataListcontainsitem {
        #[block(location = fields)]
        list: List,
        item: Expression,
    },

    SensingTouchingobject {
        #[block(json_name = "TOUCHINGOBJECTMENU")]
        object: DropdownMenuOf<SensingTouchingobject>,
    },

    SensingMousedown,
    SensingTouchingcolor {
        color: Color,
    },
    SensingKeypressed {
        key_option: DropdownMenuOf<SensingKeyoptions>,
    },
    SensingColoristouchingcolor {
        color: Color,
        color2: Color,
    },
    SensingKeyoptions {
        #[block(location = fields)]
        key_option: DirectDropdownOf<svalue::ARc<str>>,
    },
    SensingLoudness,
}
