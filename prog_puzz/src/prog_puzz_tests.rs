use std::collections::HashMap;

use macroquad::prelude::*;

use tile_engine::for_gamedata::*;
use tile_engine::for_testing::*;

use super::objs::*;

fn get_lev(n: i32) -> Pane<super::ProgpuzzMovementLogic> {
    // NB: Use progpuzz key directly
    let test_key = HashMap::from([
        (' ', vec![ new_floor() ]),
        ('#', vec![ new_floor(), new_wall() ]),
        ('^', vec![ new_floor(), new_progbot(CoordDelta::from_xy(0, -1)) ]),
        ('>', vec![ new_floor(), new_progbot(CoordDelta::from_xy(1, 0)) ]),
        ('v', vec![ new_floor(), new_progbot(CoordDelta::from_xy(0, 1)) ]),
        ('<', vec![ new_floor(), new_progbot(CoordDelta::from_xy(-1, 0)) ]),
        ('w', vec![ new_door_win() ]),
    ]);

    match n {
        1 => Pane::Split(Split::new::<16>(
            Arena::from_ascii(&[
            "##############",
            "#            #",
            "#      w     #",
            "#            #",
            "#   ^        #",
            "#            #",
            "##############",
            ], test_key),
            Code::from_ascii(
                // NB: Consider crate macro to initialise vec
                // NB: Consider my iteration macro here and elsewhere I collect'ed.
                [
                    ("F", 1),
                    ("L", 1),
                    ("R", 1),
                    ("Loop", 1),
                ].into_iter().collect()
            )
        )),
        _ => panic!(),
    }
}

#[test]
fn basic_move() {
    // TODO: Add printout of actual map on failure
    // TODO: Check coding/running changes as expected
    // TODO: Simpler syntax for test without so much input boilerplate

    let mut curr_pane_state = get_lev(1);
    let mut input = Input::new_blank();
    input.inject_cmd(Cmd::Stay);  let _ = curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#   ^        #", ); // Start running, no other effect
    input.inject_cmd(Cmd::Stay);  let _ = curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[3], "#   ^        #", ); // F
    input.inject_cmd(Cmd::Stay);  let _ = curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#   ^  w     #", ); // F
    input.inject_cmd(Cmd::Stay);  let _ = curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#   >  w     #", ); // R
    input.inject_cmd(Cmd::Stay);  let _ = curr_pane_state.advance(&mut input); assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#    > w     #", ); // F
}
