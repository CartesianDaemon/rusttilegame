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

#[derive(Clone, Copy)]
pub enum Stage {
    NewGame,
    LevIntro(u16),
    LevPlay(u16),
    LevOutro(u16),
    Retry(u16),
    Win,
}

pub fn load_newgame() -> Play {
    load_stage(Stage::NewGame)
}

pub fn load_stage(stage: Stage) -> Play {
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
        Stage::NewGame => make_splash("Press [enter] to start.".to_string(), Stage::LevIntro(1)),

        Stage::LevIntro(1) => make_splash("Welcome to level 1!".to_string(), Stage::LevPlay(1)),
        Stage::LevPlay(1) => make_levplay(1, &[
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
        Stage::LevOutro(1) => make_splash("Well done!! Goodbye from level 1".to_string(), Stage::LevIntro(2)),

        Stage::LevIntro(2) => make_splash("Ooh, welcome to level 2!".to_string(), Stage::LevPlay(2)),
        Stage::LevPlay(2) => make_levplay(2, &[
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
        Stage::LevOutro(2) => make_splash("Wow, well done!! Goodbye from level 2!".to_string(), Stage::Win),

        Stage::Retry(levno) => make_splash("Game Over. Press [enter] to retry.".to_string(), Stage::LevPlay(levno)),
        Stage::Win => make_splash("Congratulations. You win! Press [enter] to play again.".to_string(), Stage::LevIntro(1)),

        Stage::LevIntro(_) => panic!("Loading LevIntro for level that can't be found."),
        Stage::LevPlay(_) => panic!("Loading LevPlay for level that can't be found."),
        Stage::LevOutro(_) => panic!("Loading LevOutro for level that can't be found."),
    }
}

pub fn make_splash(txt: String, to_stage: Stage) -> Play {
    Play {
        mode: Mode::Splash,
        splash_text: txt,
        to_stage: to_stage,
        die_stage: Stage::NewGame, // Shouldn't be used?
        ..Play::new_null_level()
    }
}

// Used by tests
pub fn make_levplay(levno: u16, ascii_map: &[&str; 16], map_key: HashMap<char, Vec<Ent>>) -> Play {
    Play {
        mode : Mode::LevPlay,
        to_stage: Stage::LevOutro(levno),
        die_stage: Stage::Retry(levno),
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
