mod any_dropdown;
mod direct_dropdown;
mod dropdown_menu_of;
mod instances_direct;
mod instances_menu;

pub use any_dropdown::AnyDropdownOf;
pub use direct_dropdown::DirectDropdownOf;
pub use dropdown_menu_of::DropdownMenuOf;
pub use instances_direct::{
    ForwardBackward, GreaterThan, LooksEffect, LooksGotoFrontBack,
    LooksTargetNumbername as LooksBackdropnumbername,
    LooksTargetNumbername as LooksCostumenumbername, MotionRotationstyle, OperatorMathop,
    SensingCurrent, SensingDragmode, SensingOfProperty, SoundEffect,
};
pub use instances_menu::{
    ChooseClone, LooksBackdrops, LooksCostume, MotionPointtowards, PenColorParam,
    PossibleGlideToPos, PossibleGoToPos, SensingKeyoptions, SensingOfObject, SensingTouchingobject,
    SoundSounds, Text2SpeechLanguages, Text2SpeechVoices,
};

use crate::_exports::AsOpcodeUnit;

macro_rules! construct_menu_arr {
    ($($ty: ident),* $(,)?) => {
        [
        $(
        {
        use instances_menu::*;
        type X = <$ty as AsOpcodeUnit>::OpcodeUnit;
           X::$ty.k_opcode_name()
        }
        ),*
        ]
    };
}

pub const META_DROPDOWN_MENUES: [&str; 13] = construct_menu_arr![
    MotionPointtowardsMenu,
    SensingTouchingobjectmenu,
    SensingOfObjectMenu,
    MotionGlidetoMenu,
    LooksBackdropsMenu,
    MotionGotoMenu,
    ControlCreateCloneOfMenu,
    LooksCostumeMenu,
    SoundSoundsMenu,
    SensingKeyoptionsMenu,
    PenColorParamMenu,
    Text2SpeechVoicesMenu,
    Text2SpeechLanguagesMenu
];

use crate::attrs::dropdowns::direct_dropdown::FromDirectDropdownString;

impl FromDirectDropdownString for svalue::ARc<str> {
    const VALID_HINT: &'static str = "any string value";

    fn from_direct_dropdown_string(val: &str) -> Option<Self> {
        Some(val.into())
    }
}
