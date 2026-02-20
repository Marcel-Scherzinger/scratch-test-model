use scratch_test_model_proc::Block;

use crate::attrs::{
    ArgumentReporterName, Color, DirectDropdownOf, Expression, ExpressionRef, List, RefBlock,
    RoundDropdownMenuOf,
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
        object: RoundDropdownMenuOf<SensingTouchingobject>,
    },

    SensingMousedown,
    SensingTouchingcolor {
        color: either::Either<Color, ExpressionRef>,
    },
    SensingKeypressed {
        key_option: RoundDropdownMenuOf<SensingKeyoptions>,
    },
    SensingColoristouchingcolor {
        color: either::Either<Color, ExpressionRef>,
        color2: either::Either<Color, ExpressionRef>,
    },
    SensingKeyoptions {
        #[block(location = fields)]
        key_option: DirectDropdownOf<svalue::ARc<str>>,
    },
    SensingLoudness,
}
