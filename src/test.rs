use std::collections::HashMap;
use assrt::*;

use crate::load::*; // For new_xxxx() fns
use crate::play::Play;

#[cfg(test)]
mod basic_tests {
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

        make_levplay(1, &[
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

    #[macroquad::test]
    async fn basic_move() {
        let p = get_lev(1);
    }
}
