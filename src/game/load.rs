// Code for loading or instatiating each level.

use std::collections::HashMap;

use crate::game::Map;
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
    match levno {
        1 => {
            let mut play = Play {
                mode : Mode::LevIntro(1),
                splash_text: "Welcome to level 1!".to_string(),
                outro_text: "Well done!! Goodbye from level 1!".to_string(),
                ..Play::new_empty_level()
            };

            // TODO: Make sure map can be assigned by a non-16x16 level
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

            // TODO: Move definitions of specific Ents into load not map.
            // TODO: Key as in "explain which symbol is which" not in key, val.
            let map_key = HashMap::from([
                (' ', Ent::new_floor()),
                ('#', Ent::new_wall()),
                ('>', Ent::new_floor()), // TODO
                ('h', Ent::new_floor()),
            ]);

            // TODO: Get size from strings, not map. Assert compatible sizes.
            for (y, line) in ascii_map.iter().enumerate() {
                for (x, ch) in line.chars().enumerate() {
                    let ent = map_key.get(&ch).unwrap().clone();
                    play.map.set_at(x as i16, y as i16, ent); // TODO
                    // play.map.set_at(x as i16, y as i16, Ent::new_floor());
                }
            }

            play.spawn_at(3, 8, Ent::new_hero_crab());

            play.spawn_mov(1, 1, Ent::new_snake((1,0)));

            play
        }
        2 => {
            let mut play = Play {
                mode : Mode::LevIntro(2),
                splash_text: "Ooh, welcome to level 2!".to_string(),
                outro_text: "Wow, well done!! Goodbye from level 2!".to_string(),
                ..Play::new_empty_level()
            };

            add_default_floor_walls(&mut play.map);

            play.spawn_hero(3, 8, Ent::new_hero_crab());

            play.spawn_mov(1, 1, Ent::new_snake((1,0)));
            play.spawn_mov(9, 9, Ent::new_snake((-1,0)));

            play
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

fn add_default_floor_walls(map: &mut Map) {
    for (x, y) in map.coords() {
        map.set_at(x as i16, y as i16, Ent::new_floor());
        if map.is_edge(x, y) {
            map.set_at(x, y, Ent::new_wall());
        }
    }
}
