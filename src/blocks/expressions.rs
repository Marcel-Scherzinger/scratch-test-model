use crate::attrs::{
    ArgumentReporterName, DirectDropdownOf, Expression, List, RoundDropdownMenuOf,
    dropdowns::{
        LooksBackdropnumbername, LooksCostumenumbername, OperatorMathop, SensingCurrent,
        SensingDistanceto, SensingOfObject, SensingOfProperty,
    },
};
use scratch_test_model_proc::Block;

#[derive(Debug, PartialEq, Clone, Block)]
#[block(default_location = inputs)]
pub enum ExprBlockKind {
    ArgumentReporterStringNumber {
        #[block(location = fields)]
        value: ArgumentReporterName,
    },
    ArgumentReporterBoolean {
        #[block(location = fields)]
        value: ArgumentReporterName,
    },

    OperatorLength {
        string: Expression,
    },
    OperatorJoin {
        string1: Expression,
        string2: Expression,
    },
    OperatorAdd {
        num1: Expression,
        num2: Expression,
    },
    OperatorMultiply {
        num1: Expression,
        num2: Expression,
    },
    OperatorMod {
        num1: Expression,
        num2: Expression,
    },
    OperatorSubtract {
        num1: Expression,
        num2: Expression,
    },
    OperatorDivide {
        num1: Expression,
        num2: Expression,
    },
    OperatorMathop {
        #[block(location = fields)]
        operator: DirectDropdownOf<OperatorMathop>,
        num: Expression,
    },
    OperatorLetterOf {
        letter: Expression,
        string: Expression,
    },
    OperatorRound {
        num: Expression,
    },
    OperatorRandom {
        from: Expression,
        to: Expression,
    },

    SensingAnswer,

    DataItemnumoflist {
        #[block(location = fields)]
        list: List,
        item: Expression,
    },
    DataItemoflist {
        #[block(location = fields)]
        list: List,
        index: Expression,
    },
    DataLengthoflist {
        #[block(location = fields)]
        list: List,
    },
    MotionYposition,
    LooksSize,
    SensingOf {
        #[block(location = fields)]
        property: DirectDropdownOf<SensingOfProperty>,
        object: RoundDropdownMenuOf<SensingOfObject>,
    },
    SensingCurrent {
        #[block(location = fields)]
        currentmenu: DirectDropdownOf<SensingCurrent>,
    },
    LooksBackdropnumbername {
        #[block(location = fields)]
        number_name: DirectDropdownOf<LooksBackdropnumbername>,
    },
    LooksCostumenumbername {
        #[block(location = fields)]
        number_name: DirectDropdownOf<LooksCostumenumbername>,
    },
    SensingMousex,
    SensingUsername,
    SensingMousey,
    SensingTimer,
    MotionDirection,
    MotionIfonedgebounce,
    SoundVolume,
    MotionXposition,

    SensingDistanceto {
        distancetomenu: RoundDropdownMenuOf<SensingDistanceto>,
    },

    /// ```
    /// # use scratch_test_model::blocks::ExprBlockKindUnit;
    /// assert_eq!("sensing_dayssince2000", ExprBlockKindUnit::SensingDayssince2000.k_opcode_name());
    /// ```
    #[block(opcode = "sensing_dayssince2000")]
    SensingDayssince2000,
    /*
    // reading a list is not a real block, use this pseudo block
    #[block(skip, opcode = "pseudo_read_data_list")]
    RDataList {
        list: List,
    },
    // reading a variable is not a real block, use this pseudo block
    #[block(skip, opcode = "pseudo_read_data_variable")]
    RDataVar {
        variable: Variable,
    },
    */
}
