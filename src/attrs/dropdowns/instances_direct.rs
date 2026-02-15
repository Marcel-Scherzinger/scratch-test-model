use crate::attrs::dropdowns::direct_dropdown::FromDirectDropdownString;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum GreaterThan {
    Loudness,
    Timer,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ForwardBackward {
    Forward,
    Backward,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SensingDragmode {
    Draggable,
    NotDraggable,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum SensingOfProperty {
    Volume,
    Timer,
    Other(svalue::ARc<str>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum LooksEffect {
    Color,
    Fisheye,
    Whirl,
    Pixelate,
    Mosaic,
    Brightness,
    Ghost,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SoundEffect {
    Pitch,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum LooksTargetNumbername {
    Name,
    Number,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum LooksGotoFrontBack {
    Front,
    Back,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum MotionRotationstyle {
    DontRotate,
    LeftRight,
    AllAround,
}
impl FromDirectDropdownString for LooksGotoFrontBack {
    const VALID_HINT: &'static str = "either 'front' or 'back'";
    fn from_direct_dropdown_string(val: &str) -> Option<Self> {
        Some(match val {
            "front" => Self::Front,
            "back" => Self::Back,
            _ => return None,
        })
    }
}
impl FromDirectDropdownString for ForwardBackward {
    const VALID_HINT: &'static str = "either 'forward' or 'backward'";
    fn from_direct_dropdown_string(val: &str) -> Option<Self> {
        Some(match val {
            "forward" => Self::Forward,
            "backward" => Self::Backward,
            _ => return None,
        })
    }
}

impl FromDirectDropdownString for MotionRotationstyle {
    const VALID_HINT: &'static str = "";
    fn from_direct_dropdown_string(val: &str) -> Option<Self> {
        Some(match val {
            "left-right" => Self::LeftRight,
            "don't rotate" => Self::DontRotate,
            "all around" => Self::AllAround,
            _ => return None,
        })
    }
}

impl FromDirectDropdownString for LooksTargetNumbername {
    const VALID_HINT: &'static str = "either 'name' or 'number'";
    fn from_direct_dropdown_string(val: &str) -> Option<Self> {
        Some(match val {
            "name" => Self::Name,
            "number" => Self::Number,
            _ => return None,
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum SensingCurrent {
    Month,
    Year,
    Date,
    Hour,
    Minute,
    Second,
    DayOfWeek,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum OperatorMathop {
    TenRaisedTo,
    Cos,
    Sin,
    Exp,
    Abs,
    ArcusCos,
    Tan,
    Ln,
    Sqrt,
    ArcusSin,
    ArcusTan,
    Floor,
    Ceiling,
    Log,
}
impl FromDirectDropdownString for OperatorMathop {
    const VALID_HINT: &'static str = ""; // TODO
    fn from_direct_dropdown_string(val: &str) -> Option<Self> {
        Some(match val {
            "e ^" => Self::Exp,
            "abs" => Self::Abs,
            "10 ^" => Self::TenRaisedTo,
            "acos" => Self::ArcusCos,
            "asin" => Self::ArcusSin,
            "atan" => Self::ArcusTan,
            "cos" => Self::Cos,
            "sin" => Self::Sin,
            "tan" => Self::Tan,
            "sqrt" => Self::Sqrt,
            "floor" => Self::Floor,
            "ceiling" => Self::Ceiling,
            "log" => Self::Log,
            "ln" => Self::Ln,
            _ => return None,
        })
    }
}

impl FromDirectDropdownString for SensingCurrent {
    const VALID_HINT: &'static str = ""; // TODO
    fn from_direct_dropdown_string(val: &str) -> Option<Self> {
        Some(match val {
            "MONTH" => Self::Month,
            "YEAR" => Self::Year,
            "DATE" => Self::Date,
            "DAYOFWEEK" => Self::DayOfWeek,
            "HOUR" => Self::Hour,
            "MINUTE" => Self::Minute,
            "SECOND" => Self::Second,
            _ => return None,
        })
    }
}

impl FromDirectDropdownString for SensingDragmode {
    const VALID_HINT: &'static str = "either 'not draggable' or 'draggable'";
    fn from_direct_dropdown_string(val: &str) -> Option<Self> {
        Some(match val {
            "not draggable" => Self::NotDraggable,
            "draggable" => Self::Draggable,
            _ => return None,
        })
    }
}

impl FromDirectDropdownString for GreaterThan {
    const VALID_HINT: &'static str = "either 'LOUDNESS' or 'TIMER'";
    fn from_direct_dropdown_string(val: &str) -> Option<Self> {
        Some(match val {
            "LOUDNESS" => GreaterThan::Loudness,
            "TIMER" => GreaterThan::Timer,
            _ => return None,
        })
    }
}
impl FromDirectDropdownString for SoundEffect {
    const VALID_HINT: &'static str = "'PITCH', ...";
    fn from_direct_dropdown_string(val: &str) -> Option<Self> {
        Some(match val {
            "PITCH" => Self::Pitch,
            _ => return None,
        })
    }
}
impl FromDirectDropdownString for LooksEffect {
    const VALID_HINT: &'static str = "'COLOR', 'FISHEYE', ...";
    fn from_direct_dropdown_string(val: &str) -> Option<Self> {
        Some(match val {
            "COLOR" => Self::Color,
            "FISHEYE" => Self::Fisheye,
            "WHIRL" => Self::Whirl,
            "BRIGHTNESS" => Self::Brightness,
            "GHOST" => Self::Ghost,
            "PIXELATE" => Self::Pixelate,
            "MOSAIC" => Self::Mosaic,
            _ => return None,
        })
    }
}
impl FromDirectDropdownString for SensingOfProperty {
    const VALID_HINT: &'static str =
        "'volume' or 'timer' for one manifestation, multiple values for other";
    fn from_direct_dropdown_string(val: &str) -> Option<Self> {
        Some(match val {
            "volume" => Self::Volume,
            "timer" => Self::Timer,
            val => Self::Other(val.into()),
        })
    }
}
