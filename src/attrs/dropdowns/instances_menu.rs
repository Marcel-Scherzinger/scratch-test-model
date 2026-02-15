use scratch_test_model_proc::Block;

use crate::attrs::{
    DirectDropdownOf, StringAtArrayPosZero,
    dropdowns::{
        direct_dropdown::FromDirectDropdownString, dropdown_menu_of::ExtraMenuDropdownParseStrat,
    },
};

#[derive(Block)]
pub enum SensingOfObjectMenu {
    SensingOfObjectMenu {
        #[block(location = fields)]
        object: StringAtArrayPosZero,
    },
}

#[derive(Block)]
pub enum Text2SpeechVoicesMenu {
    #[block(opcode = "text2speech_menu_voices")]
    Text2SpeechVoicesMenu {
        #[block(location = fields, json_name = "voices")]
        voices: DirectDropdownOf<svalue::ARc<str>>, // TODO: specific
    },
}

#[derive(Block)]
pub enum Text2SpeechLanguagesMenu {
    #[block(opcode = "text2speech_menu_languages")]
    Text2SpeechLanguagesMenu {
        #[block(location = fields, json_name = "languages")]
        languages: DirectDropdownOf<svalue::ARc<str>>, // TODO: specific
    },
}

#[derive(Block)]
pub enum MotionPointtowardsMenu {
    MotionPointtowardsMenu {
        #[block(location = fields)]
        towards: StringAtArrayPosZero,
    },
}
#[derive(Block)]
pub enum SensingKeyoptionsMenu {
    #[block(opcode = "sensing_keyoptions")]
    SensingKeyoptionsMenu {
        #[block(location = fields)]
        key_option: StringAtArrayPosZero,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SoundSounds(svalue::ARc<str>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct LooksCostume(svalue::ARc<str>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ChooseClone(svalue::ARc<str>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SensingKeyoptions(svalue::ARc<str>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Text2SpeechVoices(svalue::ARc<str>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Text2SpeechLanguages(svalue::ARc<str>);

#[derive(Block)]
pub enum ControlCreateCloneOfMenu {
    ControlCreateCloneOfMenu {
        #[block(location = fields)]
        clone_option: DirectDropdownOf<svalue::ARc<str>>,
    },
}

#[derive(Block)]
pub enum SoundSoundsMenu {
    SoundSoundsMenu {
        #[block(location = fields)]
        sound_menu: DirectDropdownOf<svalue::ARc<str>>,
    },
}

#[derive(Block)]
pub enum LooksCostumeMenu {
    #[block(opcode = "looks_costume")]
    LooksCostumeMenu {
        #[block(location = fields)]
        costume: DirectDropdownOf<svalue::ARc<str>>,
    },
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SensingOfObject(svalue::ARc<str>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MotionPointtowards {
    Mouse,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SensingTouchingobject {
    Mouse,
    Edge,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum LooksBackdrops {
    Next,
    Previous,
    ByName(svalue::ARc<str>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum PenColorParam {
    Color,
    Brightness,
    Saturation,
    Transparency,
}

impl FromDirectDropdownString for PenColorParam {
    const VALID_HINT: &'static str = "'color', ...";
    fn from_direct_dropdown_string(val: &str) -> Option<Self> {
        Some(match val {
            "color" => Self::Color,
            "brightness" => Self::Brightness,
            "saturation" => Self::Saturation,
            "transparency" => Self::Transparency,
            _ => return None,
        })
    }
}

/// internal helper block that parses an external menu block of a motion_glideto dropdown
#[derive(Block)]
pub enum MotionGlidetoMenu {
    MotionGlidetoMenu {
        #[block(location = fields)]
        to: StringAtArrayPosZero,
    },
}
#[derive(Block)]
pub enum MotionGotoMenu {
    MotionGotoMenu {
        #[block(location = fields)]
        to: StringAtArrayPosZero,
    },
}

#[derive(Block)]
pub enum PenColorParamMenu {
    #[block(opcode = "pen_menu_colorParam")]
    PenColorParamMenu {
        #[block(location = fields, json_name = "colorParam")]
        color_param: DirectDropdownOf<PenColorParam>,
    },
}

#[derive(Block)]
pub enum LooksBackdropsMenu {
    #[block(opcode = "looks_backdrops")]
    LooksBackdropsMenu {
        #[block(location = fields)]
        backdrop: StringAtArrayPosZero,
    },
}

#[derive(Block)]
pub enum SensingTouchingobjectmenu {
    SensingTouchingobjectmenu {
        #[block(location = fields)]
        touchingobjectmenu: StringAtArrayPosZero,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum PossibleGoToPos {
    Random,
    Mouse,
}
#[derive(Debug, PartialEq, Clone)]
pub enum PossibleGlideToPos {
    Random,
    Mouse,
}
impl ExtraMenuDropdownParseStrat<PossibleGlideToPos> for PossibleGlideToPos {
    // const ATTRIBUTE_KEY: &'static str = "TO";
    // const MENU_BLOCK_OPCODE: &'static str = "motion_glideto_menu";
    // const DATA_LOCATION: crate::aux::AttrLocation = crate::aux::AttrLocation::Fields;
    const VALID_HINT: &'static str = "either '_mouse_' or '_random_'";

    type MenuBlock = MotionGlidetoMenu;

    fn from_menu_block(block: Self::MenuBlock) -> Result<Self, String>
    where
        Self: Sized,
    {
        let MotionGlidetoMenu::MotionGlidetoMenu { to } = block;
        match to.as_str() {
            "_mouse_" => Ok(Self::Mouse),
            "_random_" => Ok(Self::Random),
            _ => Err(to.take()),
        }
    }
}
impl ExtraMenuDropdownParseStrat<PossibleGoToPos> for PossibleGoToPos {
    const VALID_HINT: &'static str = "either '_mouse_' or '_random_'";

    type MenuBlock = MotionGotoMenu;

    fn from_menu_block(block: Self::MenuBlock) -> Result<Self, String>
    where
        Self: Sized,
    {
        let MotionGotoMenu::MotionGotoMenu { to } = block;
        match to.as_str() {
            "_mouse_" => Ok(Self::Mouse),
            "_random_" => Ok(Self::Random),
            _ => Err(to.take()),
        }
    }
}
impl ExtraMenuDropdownParseStrat<LooksBackdrops> for LooksBackdrops {
    // const ATTRIBUTE_KEY: &'static str = "BACKDROP";
    // const MENU_BLOCK_OPCODE: &'static str = "looks_backdrops";
    // const DATA_LOCATION: crate::aux::AttrLocation = crate::aux::AttrLocation::Fields;
    const VALID_HINT: &'static str = "'previous backdrop', 'next backdrop' or backdrop name";

    type MenuBlock = LooksBackdropsMenu;

    fn from_menu_block(block: Self::MenuBlock) -> Result<Self, String>
    where
        Self: Sized,
    {
        let LooksBackdropsMenu::LooksBackdropsMenu { backdrop } = block;
        match backdrop.as_str() {
            "next backdrop" => Ok(Self::Next),
            "previous backdrop" => Ok(Self::Previous),
            _ => Ok(Self::ByName(backdrop.take().into())),
        }
    }
}

impl ExtraMenuDropdownParseStrat<SensingOfObject> for SensingOfObject {
    // const ATTRIBUTE_KEY: &'static str = "OBJECT";
    // const MENU_BLOCK_OPCODE: &'static str = "sensing_of_object_menu";
    // const DATA_LOCATION: crate::aux::AttrLocation = crate::aux::AttrLocation::Fields;
    const VALID_HINT: &'static str = "unspecified value";

    type MenuBlock = SensingOfObjectMenu;

    fn from_menu_block(block: Self::MenuBlock) -> Result<Self, String>
    where
        Self: Sized,
    {
        let SensingOfObjectMenu::SensingOfObjectMenu { object } = block;
        Ok(Self(object.take().into()))
    }
}

impl ExtraMenuDropdownParseStrat<MotionPointtowards> for MotionPointtowards {
    // const ATTRIBUTE_KEY: &'static str = "TOWARDS";
    // const MENU_BLOCK_OPCODE: &'static str = "motion_pointtowards_menu";
    // const DATA_LOCATION: crate::aux::AttrLocation = crate::aux::AttrLocation::Fields;
    const VALID_HINT: &'static str = "'_mouse_'";

    type MenuBlock = MotionPointtowardsMenu;

    fn from_menu_block(block: Self::MenuBlock) -> Result<Self, String>
    where
        Self: Sized,
    {
        let MotionPointtowardsMenu::MotionPointtowardsMenu { towards } = block;
        match towards.as_str() {
            "_mouse_" => Ok(Self::Mouse),
            _ => Err(towards.take()),
        }
    }
}

impl ExtraMenuDropdownParseStrat<SensingTouchingobject> for SensingTouchingobject {
    // const ATTRIBUTE_KEY: &'static str = "TOUCHINGOBJECTMENU";
    // const MENU_BLOCK_OPCODE: &'static str = "sensing_touchingobjectmenu";
    // const DATA_LOCATION: crate::aux::AttrLocation = crate::aux::AttrLocation::Fields;
    const VALID_HINT: &'static str = "either '_mouse_' or '_edge_'";

    type MenuBlock = SensingTouchingobjectmenu;

    fn from_menu_block(block: Self::MenuBlock) -> Result<Self, String>
    where
        Self: Sized,
    {
        let SensingTouchingobjectmenu::SensingTouchingobjectmenu { touchingobjectmenu } = block;
        let value = match touchingobjectmenu.as_str() {
            "_mouse_" => Self::Mouse,
            "_edge_" => Self::Edge,
            _ => return Err(touchingobjectmenu.take()),
        };
        Ok(value)
    }
}

impl ExtraMenuDropdownParseStrat<SoundSounds> for SoundSounds {
    const VALID_HINT: &'static str = "name of sound";

    type MenuBlock = SoundSoundsMenu;

    fn from_menu_block(block: Self::MenuBlock) -> Result<Self, String>
    where
        Self: Sized,
    {
        let SoundSoundsMenu::SoundSoundsMenu { sound_menu } = block;

        Ok(Self(sound_menu.0.0))
    }
}

impl ExtraMenuDropdownParseStrat<LooksCostume> for LooksCostume {
    const VALID_HINT: &'static str = "name of costume";

    type MenuBlock = LooksCostumeMenu;

    fn from_menu_block(block: Self::MenuBlock) -> Result<Self, String>
    where
        Self: Sized,
    {
        let LooksCostumeMenu::LooksCostumeMenu { costume } = block;

        Ok(Self(costume.0.0))
    }
}

// TODO: limit to not-string
impl ExtraMenuDropdownParseStrat<ChooseClone> for ChooseClone {
    const VALID_HINT: &'static str = "name of object or '_myself_'";

    type MenuBlock = ControlCreateCloneOfMenu;

    fn from_menu_block(block: Self::MenuBlock) -> Result<Self, String>
    where
        Self: Sized,
    {
        let ControlCreateCloneOfMenu::ControlCreateCloneOfMenu { clone_option } = block;
        Ok(Self(clone_option.0.0))
    }
}

impl ExtraMenuDropdownParseStrat<SensingKeyoptions> for SensingKeyoptions {
    const VALID_HINT: &'static str = "";

    type MenuBlock = SensingKeyoptionsMenu;

    fn from_menu_block(block: Self::MenuBlock) -> Result<Self, String>
    where
        Self: Sized,
    {
        let SensingKeyoptionsMenu::SensingKeyoptionsMenu { key_option } = block;
        Ok(Self(key_option.0.into()))
    }
}

impl ExtraMenuDropdownParseStrat<PenColorParam> for PenColorParam {
    const VALID_HINT: &'static str = "";

    type MenuBlock = PenColorParamMenu;

    fn from_menu_block(block: Self::MenuBlock) -> Result<Self, String>
    where
        Self: Sized,
    {
        let PenColorParamMenu::PenColorParamMenu { color_param } = block;
        Ok(color_param.0.0)
    }
}

impl ExtraMenuDropdownParseStrat<Text2SpeechVoices> for Text2SpeechVoices {
    const VALID_HINT: &'static str = "";

    type MenuBlock = Text2SpeechVoicesMenu;

    fn from_menu_block(block: Self::MenuBlock) -> Result<Self, String>
    where
        Self: Sized,
    {
        let Text2SpeechVoicesMenu::Text2SpeechVoicesMenu { voices } = block;
        Ok(Self(voices.0.0))
    }
}

impl ExtraMenuDropdownParseStrat<Text2SpeechLanguages> for Text2SpeechLanguages {
    const VALID_HINT: &'static str = "";

    type MenuBlock = Text2SpeechLanguagesMenu;

    fn from_menu_block(block: Self::MenuBlock) -> Result<Self, String>
    where
        Self: Sized,
    {
        let Text2SpeechLanguagesMenu::Text2SpeechLanguagesMenu { languages } = block;
        Ok(Self(languages.0.0))
    }
}
