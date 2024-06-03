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

pub fn make_splash(txt: &str, to_stage: Stage) -> Play {
    Play {
        mode: Mode::Splash,
        splash_text: txt,
        ..Play::new_empty_level()
    }
}

pub fn make_levplay(ascii_map: &[&str; 16], map_key: HashMap<char, Vec<Ent>>) -> Play {
    Play {
        mode : Mode::LevIntro(1),
        splash_text: "Welcome to level 1!".to_string(),
        outro_text: "Well done!! Goodbye from level 1!".to_string(),
        ..Play::from_ascii(&ascii_map, aquarium1_key)
    }
    Play {
        mode: Mode::Splash,
        splash_text: txt,
        ..Play::new_empty_level()
    }
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
        NewGame => make_splash("Press [enter] to start.".to_string()),
        LevIntro(1) => make_splash("".to_string());
        Retry => make_splash("Game Over. Press [enter] to retry.".to_string()),
        Win => make_splash("Congratulations. You win! Press [enter] to play again.".to_string()),
    }
}

pub fn load_level(levno: u16) -> Play {
    // TODO: Move definitions of specific Ents into load not map.
    // TODO: Key as in "explain which symbol is which" not in key, val.
    match levno {
        1 => {
            let ascii_map = [
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
            ];

            Play {
                mode : Mode::LevIntro(1),
                splash_text: "Welcome to level 1!".to_string(),
                outro_text: "Well done!! Goodbye from level 1!".to_string(),
                ..Play::from_ascii(&ascii_map, aquarium1_key)
            }
        }
        2 => {
            let ascii_map = [
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
            ];

            Play {
                mode : Mode::LevIntro(2),
                splash_text: "Ooh, welcome to level 2!".to_string(),
                outro_text: "Wow, well done!! Goodbye from level 2!".to_string(),
                ..Play::from_ascii(&ascii_map, aquarium1_key)
            }
        }
        3 => {
            Play {
                mode : Mode::Win,
                splash_text: "Congratulations. You win! Press [enter] to play again.".to_string(),
                ..Play::new_empty_level()
            }
        }
        _ => {
            // TODO Design: Is a level-design error helpful separate from engine-logic panic?
            panic!("Unknown level");
        }
    }
}
