use scratch_test_model_proc::Block;

use crate::attrs::{
    BroadcastId, DirectDropdownOf, Expression, KeyboardKey, dropdowns::GreaterThan,
};

#[derive(Debug, PartialEq, Block, Clone)]
#[allow(clippy::enum_variant_names)]
#[block(default_location = inputs)]
pub enum EventBlockKind {
    EventWhenflagclicked,
    EventWhenthisspriteclicked,
    EventWhenbackdropswitchesto {
        #[block(location = fields)]
        backdrop: DirectDropdownOf<svalue::ARc<str>>,
    },
    EventWhenkeypressed {
        #[block(location = fields)]
        key_option: DirectDropdownOf<KeyboardKey>,
    },
    EventWhenbroadcastreceived {
        #[block(location = fields)]
        broadcast_option: BroadcastId,
    },
    EventWhengreaterthan {
        #[block(location = inputs)]
        value: Expression,
        #[block(location = fields, json_name = "WHENGREATERTHANMENU")]
        category: DirectDropdownOf<GreaterThan>,
    },
}
