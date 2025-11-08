#![allow(unused)] // TODO

// Maybe: Move some simple tests back into engine.

mod sample_objs;

use sample_objs::*;

use std::collections::HashMap;

use macroquad::prelude::*;

use tile_engine::for_gamedata::*;
use tile_engine::for_testing::*;
use tile_engine::input::*;

#[cfg(test)]
mod basic_tests {
    use super::*;

    fn get_lev(n: i32) -> Pane<super::super::pushpuzz::PushpuzzMovementLogic> {
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
            1=> Pane::from_play_ascii_map(&[
                "#####_########_#",
                "#            # #",
                "#  >         @ @",
                "#            _ #",
                "#       h    # #",
                "#            # #",
                "##############@#",
            ], test_key
            ),
            2=> Pane::from_play_ascii_map(&[
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
        let mut curr_pane_state = get_lev(1);
        assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#  >         @ @");
    }

    #[test]
    fn basic_bounce() {
        let mut curr_pane_state = get_lev(1);
        let mut input = Input::new_blank();
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#   >        @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#    >       @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#     >      @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#      >     @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#       >    @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#        >   @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#         >  @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#          > @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#           >@ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#          < @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#         <  @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#        <   @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#       <    @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#      <     @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#     <      @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#    <       @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#   <        @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#  <         @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "# <          @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#<           @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "# >          @ @");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#  >         @ @");
    }

    #[test]
    fn basic_drift() {
        // TODO: Test rotated version of map somehow
        let mut curr_pane_state = get_lev(2);
        let mut input = Input::new_blank();
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[0], "# g #"); assert_eq!(&curr_pane_state.as_ascii_rows()[1], "#   #");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[0], "#  g#"); assert_eq!(&curr_pane_state.as_ascii_rows()[1], "#   #");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[0], "#   #"); assert_eq!(&curr_pane_state.as_ascii_rows()[1], "# G #");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[0], "#   #"); assert_eq!(&curr_pane_state.as_ascii_rows()[1], "#G  #");
        input.inject_cmd(Cmd::Stay); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[0], "#   #"); assert_eq!(&curr_pane_state.as_ascii_rows()[1], "# g #");
    }

    #[test]
    fn basic_move() {
        let mut curr_pane_state = get_lev(1);
        let mut input = Input::new_blank();
        input.inject_cmd(Cmd::Stay);  curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#       h    # #");
        input.inject_cmd(Cmd::Stay);  curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#       h    # #");
        input.inject_cmd(Cmd::Right); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#        h   # #");
        input.inject_cmd(Cmd::Right); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#         h  # #");
        input.inject_cmd(Cmd::Left);  curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#        h   # #");
        input.inject_cmd(Cmd::Left);  curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#       h    # #");
        input.inject_cmd(Cmd::Right); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#        h   # #");
        input.inject_cmd(Cmd::Stay);  curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#        h   # #");
        input.inject_cmd(Cmd::Right); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#         h  # #");
        input.inject_cmd(Cmd::Right); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#          h # #");
        input.inject_cmd(Cmd::Right); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#           h# #");
        input.inject_cmd(Cmd::Right); curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#           h# #");
        input.inject_cmd(Cmd::Left);  curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#          h # #");
    }

    // TODO: Test win
    // TODO: Test die
    // TODO: Test moving two objs into same Loc

    #[test]
    fn clone_map_and_move() {
        let orig_curr_pane_state = get_lev(1);
        let mut curr_pane_state = orig_curr_pane_state.clone();
        let mut input = Input::new_blank();
        println!("Orig>> {orig_curr_pane_state:?}");
        println!("Clone>> {curr_pane_state:?}");
        input.inject_cmd(Cmd::Right); curr_pane_state.advance(&mut input);
    }
}
