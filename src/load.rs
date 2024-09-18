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

/** An instance of LevelData defines a whole game.
 *
 * Making this compile-time polymorphism is over-engineered, it would more
 * naturally be run-time polymorphism. But if we can ensure that all Stages
 * are automatically valid ones, that's nice.
 */
pub trait LevelData {
    /** A type representing "level 1 intro" or "game over screen" etc.
     *
     * May need to break this down to level (often numbered) and step in level (intro, etc).
     */
    type Stage;

    /** The stage to begin when a game launches. E.g. BiobotStage::NewGame */
    fn initial_stage() -> Self::Stage;

    /** Given a stage identifier, return a Play instance for that stage.
     *
     * Most naturally would be a hash lookup, or a "load file N from disk". But could have
     * levels generated programmatically.
     */
    fn load_stage(stage: BiobotStage) -> Play;
}

struct BioBots;

impl LevelData for BioBots {
    type Stage = BiobotStage;

    fn initial_stage() -> Self::Stage {
        BiobotStage::NewGame
    }

    fn load_stage(stage: BiobotStage) -> Play {
        biobot_load_stage(stage)
    }
}

#[derive(Clone, Copy)]
pub enum BiobotStage {
    NewGame,
    LevIntro(u16),
    LevPlay(u16),
    LevOutro(u16),
    Retry(u16),
    Win,
}

pub fn load_newgame() -> Play {
    biobot_load_stage(BiobotStage::NewGame)
}

pub fn biobot_load_stage(stage: BiobotStage) -> Play {
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
        to_stage: to_stage,
        die_stage: BiobotStage::NewGame, // Shouldn't be used?
        ..Play::new_null_level()
    }
}

// Used by tests
pub fn make_levplay(levno: u16, ascii_map: &[&str; 16], map_key: HashMap<char, Vec<Ent>>) -> Play {
    Play {
        mode : Mode::LevPlay,
        to_stage: BiobotStage::LevOutro(levno),
        die_stage: BiobotStage::Retry(levno),
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
