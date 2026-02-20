use std::collections::HashMap;

use scratch_test_model_proc::Block;

use super::procedures::procedures_call;
use crate::{
    attrs::{
        BroadcastId, Color, DirectDropdownOf, DropdownMenuOf, Expression, ExpressionRef, List,
        ProcedureArgumentId, RefBlock, RoundDropdownMenuOf, Variable,
        dropdowns::{
            ChooseClone, ForwardBackward, LooksBackdrops, LooksCostume, LooksEffect,
            LooksGotoFrontBack, MotionPointtowards, MotionRotationstyle, PenColorParam,
            PossibleGlideToPos, PossibleGoToPos, SensingDragmode, SoundEffect, SoundSounds,
            Text2SpeechLanguages, Text2SpeechVoices,
        },
    },
    blocks::{comparisons::CmpBlockKind, procedures::ProcedureId},
};

#[derive(Debug, PartialEq, Clone, Block)]
#[block(default_location = inputs)]
pub enum StmtBlockKind {
    LooksSayforsecs {
        message: Expression,
        secs: Expression,
    },
    LooksThink {
        message: Expression,
    },
    LooksThinkforsecs {
        message: Expression,
        secs: Expression,
    },
    LooksSay {
        message: Expression,
    },

    ControlWait {
        duration: Expression,
    },
    ControlIf {
        condition: Option<RefBlock<CmpBlockKind>>,
        substack: Option<RefBlock<Vec<StmtBlockKind>>>,
    },
    ControlForever {
        substack: Option<RefBlock<Vec<StmtBlockKind>>>,
    },
    ControlStop {
        #[block(location = fields)]
        stop_option: DirectDropdownOf<svalue::ARc<str>>,
    },
    ControlWaitUntil {
        condition: Option<RefBlock<CmpBlockKind>>,
    },
    ControlRepeat {
        times: Expression,
        substack: Option<RefBlock<Vec<StmtBlockKind>>>,
    },
    ControlIfElse {
        condition: Option<RefBlock<CmpBlockKind>>,
        substack: Option<RefBlock<Vec<StmtBlockKind>>>,
        substack2: Option<RefBlock<Vec<StmtBlockKind>>>,
    },
    ControlRepeatUntil {
        condition: Option<RefBlock<CmpBlockKind>>,
        substack: Option<RefBlock<Vec<StmtBlockKind>>>,
    },

    DataDeleteoflist {
        #[block(location = fields)]
        list: List,
        index: Expression,
    },
    DataDeletealloflist {
        #[block(location = fields)]
        list: List,
    },
    DataInsertatlist {
        #[block(location = fields)]
        list: List,
        index: Expression,
        item: Expression,
    },
    DataReplaceitemoflist {
        #[block(location = fields)]
        list: List,
        index: Expression,
        item: Expression,
    },
    DataAddtolist {
        #[block(location = fields)]
        list: List,
        item: Expression,
    },
    DataSetvariableto {
        #[block(location = fields)]
        variable: Variable,
        value: Expression,
    },
    DataChangevariableby {
        #[block(location = fields)]
        variable: Variable,
        value: Expression,
    },

    SensingAskandwait {
        question: Expression,
    },

    MotionGlideto {
        secs: Expression,
        to: RoundDropdownMenuOf<PossibleGlideToPos>,
    },

    LooksChangeeffectby {
        #[block(location = fields)]
        effect: DirectDropdownOf<LooksEffect>,
        change: Expression,
    },
    SensingSetdragmode {
        #[block(location = fields)]
        drag_mode: DirectDropdownOf<SensingDragmode>,
    },
    ControlDeleteThisClone,

    LooksSwitchbackdropto {
        backdrop: RoundDropdownMenuOf<LooksBackdrops>,
    },

    LooksSwitchcostumeto {
        costume: RoundDropdownMenuOf<LooksCostume>,
    },

    MotionTurnleft {
        degrees: Expression,
    },

    MotionSetrotationstyle {
        #[block(location = fields)]
        style: DirectDropdownOf<MotionRotationstyle>,
    },
    LooksGotofrontback {
        #[block(location = fields)]
        front_back: DirectDropdownOf<LooksGotoFrontBack>,
    },
    SoundPlayuntildone {
        sound_menu: RoundDropdownMenuOf<SoundSounds>,
    },
    SoundSetvolumeto {
        volume: Expression,
    },
    MotionGlidesecstoxy {
        secs: Expression,
        x: Expression,
        y: Expression,
    },
    ControlCreateCloneOf {
        clone_option: RoundDropdownMenuOf<ChooseClone>,
    },
    MotionSetx {
        x: Expression,
    },
    MotionSety {
        y: Expression,
    },
    MotionChangexby {
        dx: Expression,
    },
    MotionMovesteps {
        steps: Expression,
    },
    MotionPointindirection {
        direction: Expression,
    },
    MotionChangeyby {
        dy: Expression,
    },
    SoundCleareffects,
    SoundPlay {
        sound_menu: RoundDropdownMenuOf<SoundSounds>,
    },
    DataShowlist {
        #[block(location = fields)]
        list: List,
    },
    DataHidelist {
        #[block(location = fields)]
        list: List,
    },
    DataShowvariable {
        #[block(location = fields)]
        variable: Variable,
    },
    DataHidevariable {
        #[block(location = fields)]
        variable: Variable,
    },
    LooksShow,
    MotionPointtowards {
        towards: RoundDropdownMenuOf<MotionPointtowards>,
    },
    SoundChangevolumeby {
        volume: Expression,
    },
    SoundStopallsounds,
    LooksGoforwardbackwardlayers {
        #[block(location = fields)]
        forward_backward: DirectDropdownOf<ForwardBackward>,
        num: Expression,
    },
    MotionGotoxy {
        x: Expression,
        y: Expression,
    },
    MotionTurnright {
        degrees: Expression,
    },
    MotionGoto {
        to: RoundDropdownMenuOf<PossibleGoToPos>,
    },
    LooksNextcostume,
    LooksNextbackdrop,
    LooksHide,
    ControlStartAsClone,
    LooksSetsizeto {
        size: Expression,
    },
    LooksSeteffectto {
        #[block(location = fields)]
        effect: DirectDropdownOf<LooksEffect>,
        value: Expression,
    },
    LooksCleargraphiceffects,
    SoundChangeeffectby {
        #[block(location = fields)]
        effect: DirectDropdownOf<SoundEffect>,
        value: Expression,
    },
    SoundSeteffectto {
        #[block(location = fields)]
        effect: DirectDropdownOf<SoundEffect>,
        value: Expression,
    },
    LooksChangesizeby {
        change: Expression,
    },
    EventBroadcastandwait {
        broadcast_input: BroadcastId,
    },
    EventBroadcast {
        broadcast_input: BroadcastId,
    },

    PenClear,
    PenStamp,
    #[block(opcode = "pen_setPenColorToColor")]
    PenSetPenColorToColor {
        color: either::Either<Color, ExpressionRef>,
    },
    #[block(opcode = "pen_changePenColorParamBy")]
    PenChangePenColorParamBy {
        color_param: DropdownMenuOf<PenColorParam>,
        value: Expression,
    },
    #[block(opcode = "pen_setPenColorParamTo")]
    PenSetPenColorParamTo {
        color_param: DropdownMenuOf<PenColorParam>,
        value: Expression,
    },
    #[block(opcode = "pen_changePenSizeBy")]
    PenChangePenSizeBy {
        size: Expression,
    },
    #[block(opcode = "pen_penUp")]
    PenUp,
    #[block(opcode = "pen_penDown")]
    PenDown,
    #[block(opcode = "pen_setPenSizeTo")]
    PenSetPenSizeTo {
        size: Expression,
    },

    #[block(opcode = "procedures_call", parse_via = procedures_call)]
    ProceduresCall {
        procedure_id: ProcedureId,
        arguments: HashMap<ProcedureArgumentId, Option<Expression>>,
        warp: bool,
    },

    #[block(opcode = "text2speech_setVoice")]
    Text2SpeechSetVoice {
        voice: DropdownMenuOf<Text2SpeechVoices>,
    },
    #[block(opcode = "text2speech_setLanguage")]
    Text2SpeechSetLanguage {
        language: DropdownMenuOf<Text2SpeechLanguages>,
    },
    #[block(opcode = "text2speech_speakAndWait")]
    Text2SpeechSpeakAndWait {
        words: Expression,
    },
}

//
/*
    skip => {
        ("procedures_call")  ProceduresCall {
            // argument_values: HashMap<Id, Expression>,
            proccode: ARc<str>,
            // argumentids: ARc<[ARc<str>]>,
            arguments: ARc<[(Id, Option<Expression>)]>
        },
    },
*/
