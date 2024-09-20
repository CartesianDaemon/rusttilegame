// Code for loading or instatiating each level.
// The parts specific to this game rather than the engine.

use macroquad::prelude::*;

use std::collections::HashMap;

use crate::*;

use types::Delta;
// Need many of the specific params in ent.
// Some of those may move to his file.
use ent::*;
use play::Play;
use play::Mode;
use util::*;

/** Opaque base type for LevelSetDerived types
 *
 * A separate type because trait object types can't be copy but the derived types
 * should be. Is that necessary?
 *
 * Can I reduce the boilerplate in needing to trivially derive from both LevBase
 * and LevDerived?
 */
pub trait LevStageBase : downcast_rs::Downcast {
}
downcast_rs::impl_downcast!(LevStageBase);

/** Identify a level within a level set.
 *
 * Stored in the game engine as a box dyn trait object because each type of level
 * set can use a different type to identify levels. Typically an enum combining
 * level number with intro/play/outro, and a few special states like GameOver.
 * For level sets loaded dynamically from a file, will use a general type like
 * a string.
 */
pub trait LevStageDerived : LevStageBase + Copy + Clone {
}

/** A set of levels constituting one game.
 *
 * Built in level sets typically have no state. May implement a dynamic level
 * set which loads from a file, where an instance would contain the file name,
 * or contents from the file.
 */
pub trait LevSet {
    type LevStage : LevStageDerived;

    /** Level stage to begin game with */
    fn initial_lev_stage(&self) -> Self::LevStage;

    /** Load or construct a Play instance for the specified level stage. */
    fn _load_lev_stage(&self, lev_stage : Self::LevStage) -> Play;

    /** Load or construct a Play instance for the specified level stage.
     *
     * Default implementation downcasts a LevStageBase ref to the actual LevStage
     * type and delegates the actual work to _load_lev_stage.
     *
     * Must accept box to do the downcasting.
     *
     * Accepts ref to box. Why can't we borrow a box?
     */
    fn load_lev_stage(&self, lev_stage_box : &Box<dyn LevStageBase>) -> Play {
        if let Some(lev_stage) = lev_stage_box.downcast_ref::<Self::LevStage>() {
            self._load_lev_stage(*lev_stage)
        } else {
            panic!("Lev stage box -> lev stage cast failure");
        }
    }
}

//////////////////////////////////////////////////
// Level set for biobot game. Move to separate file

#[derive(Clone, Copy)]
pub enum BiobotStage {
    NewGame,
    LevIntro(u16),
    LevPlay(u16),
    LevOutro(u16),
    Retry(u16),
    Win,
}

impl LevStageBase for BiobotStage {
}

impl LevStageDerived for BiobotStage {
}

pub struct BiobotLevs {
    // No state needed
}

impl LevSet for BiobotLevs {
    type LevStage = BiobotStage;

    fn initial_lev_stage(&self) -> BiobotStage {
        BiobotStage::NewGame
    }

    fn _load_lev_stage(&self, lev_stage : BiobotStage) -> Play {
        biobot_load_stage_impl(lev_stage)
    }
}

pub fn biobot_load_stage(lev_stage_box : &Box<dyn LevStageBase>) -> Play {
    if let Some(lev_stage) = lev_stage_box.downcast_ref::<BiobotStage>() {
        biobot_load_stage_impl(*lev_stage)
    } else {
        panic!("Lev stage box -> lev stage cast failure");
    }
}

//impl LevSet for BiobotLevs
pub fn load_newgame() -> Play {
    biobot_load_stage_impl(BiobotStage::NewGame)
}

// Needed to make stage into reference to get downcast to work, as ref from Box couldn't be *'d
pub fn biobot_load_stage_impl(stage: BiobotStage) -> Play {
    let aquarium1_key = HashMap::from([
        (' ', vec![ new_floor() ]),
        ('#', vec![ new_floor(), new_wall() ]),
        ('>', vec![ new_floor(), new_snake((1,0)) ]),
        ('<', vec![ new_floor(), new_snake((-1,0)) ]),
        ('h', vec![ new_floor(), new_hero_crab() ]),
        ('o', vec![ /* new_floor(), */ new_door_win() ]), // TODO: Check win on non-floor tiles
        ('@', vec![ new_floor(), new_door_closed() ]),
        ('_', vec![ new_floor(), new_door_open() ]),
        /*
        */
    ]);

    match stage {
        // TODO: Can we use idx++ instead of specifying each level number? Not immediately?
        BiobotStage::NewGame => make_splash("Press [enter] to start.".to_string(), BiobotStage::LevIntro(1)),

        BiobotStage::LevIntro(1) => make_splash("Welcome to level 1!".to_string(), BiobotStage::LevPlay(1)),
        BiobotStage::LevPlay(1) => make_levplay(1, &[
            "#            # #",
            "#####@####@###@#",
            "@              #",
            "#####_########_#",
            "#            # #",
            "#            # #",
            "#  >         @ @",
            "#            # #",
            "#            # #",
            "#       h    # #",
            "#            # o",
            "#            # #",
            "#            # #",
            "##############@#",
            "#            # #",
            "#            @ #",
        ], aquarium1_key),
        BiobotStage::LevOutro(1) => make_splash("Well done!! Goodbye from level 1".to_string(), BiobotStage::LevIntro(2)),

        BiobotStage::LevIntro(2) => make_splash("Ooh, welcome to level 2!".to_string(), BiobotStage::LevPlay(2)),
        BiobotStage::LevPlay(2) => make_levplay(2, &[
            "################",
            "#              #",
            "#              #",
            "#              #",
            "#       h      #",
            "#              #",
            "#              #",
            "#  >           #",
            "#              #",
            "#        <     #",
            "#              #",
            "#              #",
            "#              #",
            "#              #",
            "#              #",
            "####o###########",
        ], aquarium1_key),
        BiobotStage::LevOutro(2) => make_splash("Wow, well done!! Goodbye from level 2!".to_string(), BiobotStage::Win),

        BiobotStage::Retry(levno) => make_splash("Game Over. Press [enter] to retry.".to_string(), BiobotStage::LevPlay(levno)),
        BiobotStage::Win => make_splash("Congratulations. You win! Press [enter] to play again.".to_string(), BiobotStage::LevIntro(1)),

        BiobotStage::LevIntro(_) => panic!("Loading LevIntro for level that can't be found."),
        BiobotStage::LevPlay(_) => panic!("Loading LevPlay for level that can't be found."),
        BiobotStage::LevOutro(_) => panic!("Loading LevOutro for level that can't be found."),
    }
}

// Replace with Play constructor directly, or if useful make this a Play fn not a Stage fn?
pub fn make_splash(txt: String, to_stage: BiobotStage) -> Play {
    Play {
        mode: Mode::Splash,
        splash_text: txt,
        to_stage: Box::new(to_stage),
        die_stage: Box::new(BiobotStage::NewGame), // Shouldn't be used?
        ..Play::new_null_level()
    }
}

// Used by tests
pub fn make_levplay(levno: u16, ascii_map: &[&str; 16], map_key: HashMap<char, Vec<Ent>>) -> Play {
    Play {
        mode : Mode::LevPlay,
        to_stage: Box::new(BiobotStage::LevOutro(levno)),
        die_stage: Box::new(BiobotStage::Retry(levno)),
        ..Play::from_ascii(&ascii_map, map_key)
    }
}

// SPECIFIC ENT TYPES
// public only for helper use in test.rs

pub fn new_hero_crab() -> Ent {
    Ent {
        pass: Pass::Mov,
        ai: AI::Hero,
        ..Ent::new_tex_col(load_texture_blocking_unwrap("imgs/ferris.png"), GOLD)
    }
}

pub fn new_snake(dir: Delta) -> Ent {
    Ent {
        pass: Pass::Mov,
        ai: AI::Bounce,
        dir: dir,
        effect: Effect::Kill,
        ..Ent::new_col(DARKGREEN)
    }
}

pub fn new_floor() -> Ent {
    Ent {
        ..Ent::new_col_outline(WHITE, LIGHTGRAY)
    }
}

pub fn new_wall() -> Ent {
    Ent {
        pass: Pass::Solid,
        ..Ent::new_col(DARKGRAY)
    }
}

pub fn new_door_open() -> Ent {
    Ent {
        ..Ent::new_col(LIGHTGRAY)
    }
}

pub fn new_door_closed() -> Ent {
    Ent {
        pass: Pass::Solid,
        ..Ent::new_col_outline(DARKGRAY, LIGHTGRAY)
    }
}

pub fn new_door_win() -> Ent {
    Ent {
        effect: Effect::Win,
        ..Ent::new_col_outline(GOLD, LIGHTGRAY)
    }
}
