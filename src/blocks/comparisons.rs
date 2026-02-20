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
    OperatorNot {
        /// reference to boolean expression that should be negated
        operand: Option<RefBlock<CmpBlockKind>>,
    },
    OperatorGt {
        operand1: Expression,
        operand2: Expression,
    },
    OperatorLt {
        operand1: Expression,
        operand2: Expression,
    },
    OperatorEquals {
        operand1: Expression,
        operand2: Expression,
    },
    OperatorContains {
        string1: Expression,
        string2: Expression,
    },

    DataListcontainsitem {
        #[block(location = fields)]
        list: List,
        item: Expression,
    },

    SensingColoristouchingcolor {
        color: either::Either<Color, ExpressionRef>,
        color2: either::Either<Color, ExpressionRef>,
    },
    SensingKeyoptions {
        #[block(location = fields)]
        key_option: DirectDropdownOf<svalue::ARc<str>>,
    },
    SensingKeypressed {
        key_option: RoundDropdownMenuOf<SensingKeyoptions>,
    },
    SensingLoudness,
    SensingMousedown,
    SensingTouchingobject {
        #[block(json_name = "TOUCHINGOBJECTMENU")]
        object: RoundDropdownMenuOf<SensingTouchingobject>,
    },
    SensingTouchingcolor {
        color: either::Either<Color, ExpressionRef>,
    },
}
