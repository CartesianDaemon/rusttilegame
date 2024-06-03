// Code for loading or instatiating each level.

use std::collections::HashMap;

// use crate::game::Map;
use crate::game::Ent;
use crate::game::Play;
use crate::game::Mode;

#[allow(dead_code)]
enum Stage {
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
        (' ', vec![ Ent::new_floor() ]),
        ('#', vec![ Ent::new_floor(), Ent::new_wall() ]),
        ('>', vec![ Ent::new_floor(), Ent::new_snake((1,0)) ]),
        ('<', vec![ Ent::new_floor(), Ent::new_snake((-1,0)) ]),
        ('h', vec![ Ent::new_floor(), Ent::new_hero_crab() ]),
    ]);

    match stage {
        // TODO: Can we use idx++ instead of specifying each level number?
        Stage::NewGame => make_splash("Press [enter] to start.".to_string(), Stage::LevIntro(1)),

        Stage::LevIntro(1) => make_splash("Welcome to level 1!".to_string(), LevPlay(1));
        Stage::LevPlay(1) => make_levplay(1, [
            "################",
            "#              #",
            "# >            #",
            "#              #",
            "#              #",
            "#              #",
            "#              #",
            "#              #",
            "#   h          #",
            "#              #",
            "#              #",
            "#              #",
            "#              #",
            "#              #",
            "#              #",
            "################",
        ], aquarium1_key);
        Stage::LevOutro(1) => make_splash("Well done!! Goodbye from level 1".to_string(), LevPlay(2));

        Stage::LevIntro(2) => make_splash("Ooh, welcome to level 2!".to_string(), LevPlay(1));
        Stage::LevPlay(2) => make_levplay(2, [
            "################",
            "#              #",
            "# >            #",
            "#              #",
            "#              #",
            "#              #",
            "#              #",
            "#              #",
            "#   h          #",
            "#        <     #",
            "#              #",
            "#              #",
            "#              #",
            "#              #",
            "#              #",
            "################",
        ], aquarium1_key);
        Stage::LevOutro(1) => make_splash("Wow, well done!! Goodbye from level 2!".to_string(), LevPlay(2));

        Stage::Retry(levno) => make_splash("Game Over. Press [enter] to retry.".to_string(), Stage::LevPlay(levno)),
        Stage::Win => make_splash("Congratulations. You win! Press [enter] to play again.".to_string(), Stage::LevIntro(1)),
    }
}

pub fn make_splash(txt: &str, to_stage: Stage) -> Play {
    Play {
        mode: Mode::Splash,
        splash_text: txt,
        to_stage: to_stage,
        die_stage: Stage::NewGame, // Shouldn't be used?
        ..Play::new_empty_level()
    }
}

pub fn make_levplay(levno: u16, ascii_map: &[&str; 16], map_key: HashMap<char, Vec<Ent>>) -> Play {
    Play {
        mode : Mode::LevPlay,
        to_stage: Stage::LevOutro(levno),
        die_stage: Stage::Retry(levno),
        ..Play::from_ascii(&ascii_map, aquarium1_key)
    }
}

