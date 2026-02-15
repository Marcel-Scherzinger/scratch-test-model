use scratch_test_model::ProjectDoc;

#[test]
fn read_all_standard_blocks() {
    let json = scratch_test_model::json_from_sb3_file("sb3/all-predefined-blocks.sb3").unwrap();
    let res = ProjectDoc::from_json(&json);
    if let Some(err) = res.err() {
        if let scratch_test_model::error::ModelError::Target(err) = err {
            panic!("{:#?}", err.inner_error());
        }

        panic!("{err:#?}");
    }
}

#[test]
fn read_custom_blocks() {
    let json = scratch_test_model::json_from_sb3_file("sb3/my-blocks.sb3").unwrap();
    let res = ProjectDoc::from_json(&json);
    if let Some(err) = res.err() {
        if let scratch_test_model::error::ModelError::Target(err) = err {
            panic!("{:#?}", err.inner_error());
        }

        panic!("{err:#?}");
    }
}

/*
const ALL_UNSUPPORTED_STD_BLOCKS: [UnsupportedBlockKind; 75] = [
    UnsupportedBlockKind::EventWhenthisspriteclicked,
    UnsupportedBlockKind::EventBroadcast,
    UnsupportedBlockKind::EventBroadcastandwait,
    UnsupportedBlockKind::EventWhenbackdropswitchesto,
    UnsupportedBlockKind::EventWhenbroadcastreceived,
    UnsupportedBlockKind::EventWhengreaterthan,
    UnsupportedBlockKind::LooksSize,
    UnsupportedBlockKind::LooksChangeeffectby,
    UnsupportedBlockKind::LooksBackdropnumbername,
    UnsupportedBlockKind::LooksCostume,
    UnsupportedBlockKind::LooksCostumenumbername,
    UnsupportedBlockKind::LooksCleargraphiceffects,
    UnsupportedBlockKind::LooksBackdrops,
    UnsupportedBlockKind::LooksChangesizeby,
    UnsupportedBlockKind::LooksGoforwardbackwardlayers,
    UnsupportedBlockKind::LooksGotofrontback,
    UnsupportedBlockKind::LooksHide,
    UnsupportedBlockKind::LooksNextbackdrop,
    UnsupportedBlockKind::LooksNextcostume,
    UnsupportedBlockKind::LooksSeteffectto,
    UnsupportedBlockKind::LooksSetsizeto,
    UnsupportedBlockKind::LooksSwitchbackdropto,
    UnsupportedBlockKind::LooksSwitchcostumeto,
    UnsupportedBlockKind::MotionGlideto,
    UnsupportedBlockKind::MotionSetrotationstyle,
    UnsupportedBlockKind::MotionChangexby,
    UnsupportedBlockKind::MotionChangeyby,
    UnsupportedBlockKind::MotionSetx,
    UnsupportedBlockKind::MotionGotoxy,
    UnsupportedBlockKind::MotionGotoMenu,
    UnsupportedBlockKind::MotionPointtowards,
    UnsupportedBlockKind::MotionDirection,
    UnsupportedBlockKind::MotionGlidesecstoxy,
    UnsupportedBlockKind::MotionGlidetoMenu,
    UnsupportedBlockKind::MotionGoto,
    UnsupportedBlockKind::MotionIfonedgebounce,
    UnsupportedBlockKind::MotionMovesteps,
    UnsupportedBlockKind::MotionPointindirection,
    UnsupportedBlockKind::MotionPointtowardsMenu,
    UnsupportedBlockKind::MotionSety,
    UnsupportedBlockKind::MotionTurnleft,
    UnsupportedBlockKind::MotionTurnright,
    UnsupportedBlockKind::MotionXposition,
    UnsupportedBlockKind::MotionYposition,
    UnsupportedBlockKind::ControlDeleteThisClone,
    UnsupportedBlockKind::ControlCreateCloneOf,
    UnsupportedBlockKind::ControlCreateCloneOfMenu,
    UnsupportedBlockKind::ControlStartAsClone,
    UnsupportedBlockKind::SoundSoundsMenu,
    UnsupportedBlockKind::SoundVolume,
    UnsupportedBlockKind::SoundChangeeffectby,
    UnsupportedBlockKind::SoundChangevolumeby,
    UnsupportedBlockKind::SoundCleareffects,
    UnsupportedBlockKind::SoundPlay,
    UnsupportedBlockKind::SoundPlayuntildone,
    UnsupportedBlockKind::SoundSeteffectto,
    UnsupportedBlockKind::SoundSetvolumeto,
    UnsupportedBlockKind::SoundStopallsounds,
    UnsupportedBlockKind::SensingKeypressed,
    UnsupportedBlockKind::SensingTouchingcolor,
    UnsupportedBlockKind::SensingLoudness,
    UnsupportedBlockKind::SensingColoristouchingcolor,
    UnsupportedBlockKind::SensingDayssince2000,
    UnsupportedBlockKind::SensingCurrent,
    UnsupportedBlockKind::SensingTimer,
    UnsupportedBlockKind::SensingOf,
    UnsupportedBlockKind::SensingOfObjectMenu,
    UnsupportedBlockKind::SensingKeyoptions,
    UnsupportedBlockKind::SensingMousedown,
    UnsupportedBlockKind::SensingMousex,
    UnsupportedBlockKind::SensingMousey,
    UnsupportedBlockKind::SensingSetdragmode,
    UnsupportedBlockKind::SensingTouchingobject,
    UnsupportedBlockKind::SensingTouchingobjectmenu,
    UnsupportedBlockKind::SensingUsername,
];
*/
