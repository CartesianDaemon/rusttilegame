#![allow(unused)] // TODO

use std::collections::HashMap;

use assrt::*;
use macroquad::prelude::*;

use crate::levset::*; // For new_xxxx() fns
use crate::levset_biobot::biobot_levplay;
use crate::play::Play;

#[cfg(test)]
mod basic_tests {
    use crate::input::Input;

    use super::*;

    #[test]
    fn basic_test() {
        rsst!(true);
    }

    fn get_lev(n: i32) -> Play {
        let test_key = HashMap::from([
            (' ', vec![ new_floor() ]),
            ('#', vec![ new_floor(), new_wall() ]),
            ('>', vec![ new_floor(), new_snake((1,0)) ]),
            ('<', vec![ new_floor(), new_snake((-1,0)) ]),
            ('h', vec![ new_floor(), new_hero_crab() ]),
            ('o', vec![ new_door_win() ]),
            ('@', vec![ new_floor(), new_door_closed() ]),
            ('_', vec![ new_floor(), new_door_open() ]),
        ]);

        biobot_levplay(1, &[
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
            ], test_key
        )
    }

    #[test]
    fn basic_move() {
        let mut play_state = get_lev(1);
        play_state.advance(&mut Input::from_key(KeyCode::Right));
    }

    #[test]
    fn clone_map_and_move() {
        let orig_play_state = get_lev(1);
        let mut play_state = orig_play_state.clone();
    }
}
