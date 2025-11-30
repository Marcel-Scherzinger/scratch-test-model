use crate::attr::{DropdownSelection, Expression, List, RefBlock, Variable};
use crate::{ARc, CmpBlockKind, Id};

super::define_blocks! {
    #[derive(Debug, PartialEq, Clone)]
    pub enum StmtBlockKind (StmtBlockKindUnit):

    "looks_sayforsecs" => LooksSayforsecs {
        message: Expression,
        secs: Expression,
    },
    "looks_think" => LooksThink {
        message: Expression,
    },
    "looks_thinkforsecs" => LooksThinkforsecs {
        message: Expression,
        secs: Expression,
    },
    "looks_say" => LooksSay {
        message: Expression,
    },

    "control_wait" => ControlWait {
        duration: Expression,
    },
    "control_if" => ControlIf {
        condition: Option<RefBlock<CmpBlockKind>>,
        substack: Option<RefBlock<Vec<StmtBlockKind>>>,
    },
    "control_forever" => ControlForever {
        substack: Option<RefBlock<Vec<StmtBlockKind>>>,
    },
    "control_stop" => ControlStop {
        (field) stop_option: DropdownSelection,
    },
    "control_wait_until" => ControlWaitUntil {
        condition: Option<RefBlock<CmpBlockKind>>,
    },
    "control_repeat" => ControlRepeat {
        times: Expression,
        substack: Option<RefBlock<Vec<StmtBlockKind>>>,
    },
    "control_if_else" => ControlIfElse {
        condition: Option<RefBlock<CmpBlockKind>>,
        substack: Option<RefBlock<Vec<StmtBlockKind>>>,
        substack2: Option<RefBlock<Vec<StmtBlockKind>>>,
    },
    "control_repeat_until" => ControlRepeatuntil {
        condition: Option<RefBlock<CmpBlockKind>>,
        substack: Option<RefBlock<Vec<StmtBlockKind>>>,
    },

    "data_deleteoflist" => DataDeleteoflist {
        (field) list: List,
        index: Expression,
    },
    "data_deletealloflist" => DataDeletealloflist {
        (field) list: List,
    },
    "data_insertatlist" => DataInsertatlist {
        (field) list: List,
        index: Expression,
        item: Expression,
    },
    "data_replaceitemoflist" => DataReplaceitemoflist {
        (field) list: List,
        index: Expression,
        item: Expression,
    },
    "data_addtolist" => DataAddtolist {
        (field) list: List,
        item: Expression,
    },
    "data_setvariableto" => DataSetvariableto {
        (field) variable: Variable,
        value: Expression
    },
    "data_changevariableby" => DataChangevariableby {
        (field) variable: Variable,
        value: Expression,
    },

    "sensing_askandwait" => SensingAskandwait {
        question: Expression,
    },

    skip => {
        ("procedures_call")  ProceduresCall {
            // argument_values: HashMap<Id, Expression>,
            proccode: ARc<str>,
            // argumentids: ARc<[ARc<str>]>,
            arguments: ARc<[(Id, Option<Expression>)]>
        },
    },
}
