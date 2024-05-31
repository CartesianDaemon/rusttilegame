// Code for loading or instatiating each level.

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

            // Initialise Floor
            // TODO: Make common helper functions.
            {
                for (x, y) in play.map.coords() {
                    play.map.set_at(x as i16, y as i16, Ent::new_floor());
                    if play.map.is_edge(x, y) {
                        play.map.set_at(x, y, Ent::new_wall());
                    }
                }
            }

            // Initialise hero
            play.spawn_hero(3, 8, Ent::new_hero_crab());

            // Initialise snake
            play.spawn_mov(1, 1, Ent::new_snake((1,0)));

            play
        }
        2 => {
            let mut play = Play {
                mode : Mode::LevIntro(1),
                splash_text: "Ooh, welcome to level 2!".to_string(),
                outro_text: "Wow, well done!! Goodbye from level 2!".to_string(),
                ..Play::new_empty_level()
            };

            // Initialise Floor
            // TODO: Make common helper functions.
            {
                for (x, y) in play.map.coords() {
                    play.map.set_at(x as i16, y as i16, Ent::new_floor());
                    if play.map.is_edge(x, y) {
                        play.map.set_at(x, y, Ent::new_wall());
                    }
                }
            }

            // Initialise hero
            play.spawn_hero(3, 8, Ent::new_hero_crab());

            // Initialise snake
            play.spawn_mov(1, 1, Ent::new_snake((1,0)));
            play.spawn_mov(9, 9, Ent::new_snake((-1,0)));

            play
        }
        3 => {
            Play {
                mode : Mode::NewGame,
                // TODO: Implement less ad hoc new game.
                splash_text: "Congratulations. You win!\n\nPress [enter] to play again.".to_string(),
                ..Play::new_empty_level()
            }
        }
        _ => {
            // TODO: Does it help to handle game-logic-errors differently to engine-logic errors?
            panic!("Unknown level");
        }
    }
}
