use std::collections::HashMap;

use macroquad::prelude::*;

use tile_engine::for_gamedata::*;
use tile_engine::for_testing::*;

use super::objs::*;

fn get_lev(n: i32) -> Pane<super::ProgpuzzMovementLogic> {
    let test_key = HashMap::from([
        (' ', vec![ new_floor() ]),
        ('#', vec![ new_floor(), new_wall() ]),
        ('^', vec![ new_floor(), new_progbot(CoordDelta::from_xy(0, -1)) ]),
        ('>', vec![ new_floor(), new_progbot(CoordDelta::from_xy(1, 0)) ]),
        ('v', vec![ new_floor(), new_progbot(CoordDelta::from_xy(0, -1)) ]),
        ('<', vec![ new_floor(), new_progbot(CoordDelta::from_xy(-1, 0)) ]),
        ('w', vec![ new_door_win() ]),
    ]);

    match n {
        1=> Pane::from_play_ascii_map(&[
            "##############",
            "#            #",
            "#      w     #",
            "#            #",
            "#   ^        #",
            "#            #",
            "##############",
        ], test_key
        ),
        _ => panic!(),
    }
}

#[test]
fn basic_move() {
    // TODO: Test program runs as expected..

    let mut curr_pane_state = get_lev(1);
    let mut input = Input::new_blank();
    input.inject_cmd(Cmd::Stay);  let _ = curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#   ^        #"); // Start running, no other effect
    input.inject_cmd(Cmd::Stay);  let _ = curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[3], "#   ^        #"); // F
    input.inject_cmd(Cmd::Stay);  let _ = curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#   ^  w     #"); // F
    input.inject_cmd(Cmd::Stay);  let _ = curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#   >  w     #"); // R
    input.inject_cmd(Cmd::Stay);  let _ = curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#    > w     #"); // F
}
