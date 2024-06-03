// Code for loading or instatiating each level.

use std::collections::HashMap;

// use crate::game::Map;
use crate::game::Ent;
use crate::game::Play;
use crate::game::Mode;

pub fn load_newgame() -> Play {
    Play {
        mode : Mode::NewGame,
        splash_text: "Press [enter] to start.".to_string(),
        ..Play::new_empty_level()
    }
}

pub fn load_retry(levno: u16) -> Play {
    Play {
        mode : Mode::Retry(levno),
        splash_text: "Game Over. Press [enter] to retry.".to_string(),
        ..Play::new_empty_level()
    }
}
 
pub fn load_level(levno: u16) -> Play {
    // TODO: Move definitions of specific Ents into load not map.
    // TODO: Key as in "explain which symbol is which" not in key, val.
    let aquarium1_key = HashMap::from([
        (' ', vec![ Ent::new_floor() ]),
        ('#', vec![ Ent::new_floor(), Ent::new_wall() ]),
        ('>', vec![ Ent::new_floor(), Ent::new_snake((1,0)) ]),
        ('<', vec![ Ent::new_floor(), Ent::new_snake((-1,0)) ]),
        ('h', vec![ Ent::new_floor(), Ent::new_hero_crab() ]),
    ]);

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
