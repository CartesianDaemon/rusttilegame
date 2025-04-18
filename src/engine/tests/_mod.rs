#![allow(unused)] // TODO

mod sample_objs;

use sample_objs::*;

use std::collections::HashMap;

use assrt::*;
use macroquad::prelude::*;

use crate::engine::gametrait::*; // For new_xxxx() fns
use crate::engine::scene::Scene;
use crate::engine::map_coords::*;

#[cfg(test)]
mod basic_tests {
    use crate::engine::input::Input;

    use super::*;

    /// TODO: Could compare whole map?
    fn expect_eq(got: &String, expected: &str) {
        if got != expected {
            panic!("Got, expected:\n{got}\n{expected}");
        }
    }

    #[test]
    fn basic_compare() {
        rsst!(true);
    }

    fn get_lev(n: i32) -> Scene {
        let test_key = HashMap::from([
            (' ', vec![ new_floor() ]),
            ('#', vec![ new_floor(), new_wall() ]),
            ('>', vec![ new_floor(), new_fish(CoordDelta::from_xy(1,0)) ]),
            ('<', vec![ new_floor(), new_fish(CoordDelta::from_xy(-1,0)) ]),
            ('g', vec![ new_floor(), new_gawpie(CoordDelta::from_xy(1,0)) ]),
            ('G', vec![ new_floor(), new_gawpie(CoordDelta::from_xy(-1,0)) ]),
            ('h', vec![ new_floor(), new_hero_crab() ]),
            ('o', vec![ new_door_win() ]),
            ('@', vec![ new_floor(), new_door_closed() ]),
            ('_', vec![ new_floor(), new_door_open() ]),
        ]);

        match n {
            1=> Scene::from_play_ascii_map(&[
                "#####_########_#",
                "#            # #",
                "#  >         @ @",
                "#            _ #",
                "#       h    # #",
                "#            # #",
                "##############@#",
            ], test_key
            ),
            2=> Scene::from_play_ascii_map(&[
                "#g  #",
                "#   #",
                "h   #",
            ], test_key
            ),
            _ => panic!(),
        }
    }

    #[test]
    fn basic_init() {
        let mut play_state = get_lev(1);
        expect_eq(&play_state.as_ascii_rows()[2], "#  >         @ @");
    }

    #[test]
    fn basic_bounce() {
        let mut play_state = get_lev(1);
        let key = &mut Input::from_key(KeyCode::Space);
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#   >        @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#    >       @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#     >      @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#      >     @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#       >    @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#        >   @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#         >  @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#          > @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#           >@ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#          < @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#         <  @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#        <   @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#       <    @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#      <     @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#     <      @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#    <       @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#   <        @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#  <         @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "# <          @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#<           @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "# >          @ @");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[2], "#  >         @ @");
    }

    #[test]
    fn basic_drift() {
        // TODO: Test rotated version of map somehow
        let mut play_state = get_lev(2);
        let key = &mut Input::from_key(KeyCode::Space);
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[0], "# g #"); expect_eq(&play_state.as_ascii_rows()[1], "#   #");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[0], "#  g#"); expect_eq(&play_state.as_ascii_rows()[1], "#   #");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[0], "#   #"); expect_eq(&play_state.as_ascii_rows()[1], "# G #");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[0], "#   #"); expect_eq(&play_state.as_ascii_rows()[1], "#G  #");
        play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[0], "#   #"); expect_eq(&play_state.as_ascii_rows()[1], "# g #");
    }

    #[test]
    fn basic_move() {
        // TODO: Need to simplify how keypress is "used up"
        let mut play_state = get_lev(1);
        let key = &mut Input::from_key(KeyCode::Right);
        let key = &mut Input::from_key(KeyCode::Right); play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[4], "#        h   # #");
        let key = &mut Input::from_key(KeyCode::Right); play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[4], "#         h  # #");
        let key = &mut Input::from_key(KeyCode::Right); play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[4], "#          h # #");
        let key = &mut Input::from_key(KeyCode::Right); play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[4], "#           h# #");
        let key = &mut Input::from_key(KeyCode::Right); play_state.advance(key); expect_eq(&play_state.as_ascii_rows()[4], "#           h# #");
    }

    // TODO: Test win
    // TODO: Test die

    #[test]
    fn clone_map_and_move() {
        let orig_play_state = get_lev(1);
        let mut play_state = orig_play_state.clone();
        println!("Orig>> {orig_play_state:?}");
        println!("Clone>> {play_state:?}");
        play_state.advance(&mut Input::from_key(KeyCode::Right));
    }
}
