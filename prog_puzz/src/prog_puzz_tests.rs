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
    // TODO: Move assert line into function. With some way of seeing how many ticks have passed.
    // NB: Get away from as_play. Instead have initial map with 0123 in, and fn to say which is ^, or other mov.
    // Then check that curr_pane_state is exactly the Pane::Something(Something) we expect.
    // Need to have decent visualisation for Pane::Something(Something).
    // Including checking that we move through level transitions ok.

    let mut curr_pane_state = get_lev(1);
    assert_eq!(curr_pane_state.advance(&mut Input::from_cmd(Cmd::Stay)), PaneContinuation::Continue(()));
    assert_eq!(&curr_pane_state.as_ascii_rows()[4], "#   ^        #", "\n{}", curr_pane_state.as_ascii_rows().join("\n")); // Start running, no other effect
    assert_eq!(curr_pane_state.advance(&mut Input::from_cmd(Cmd::Stay)), PaneContinuation::Continue(()));
    assert_eq!(&curr_pane_state.as_ascii_rows()[3], "#   ^        #", "\n{}", curr_pane_state.as_ascii_rows().join("\n")); // F
    assert_eq!(curr_pane_state.advance(&mut Input::from_cmd(Cmd::Stay)), PaneContinuation::Continue(()));
    assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#   ^  w     #", "\n{}", curr_pane_state.as_ascii_rows().join("\n")); // F
    assert_eq!(curr_pane_state.advance(&mut Input::from_cmd(Cmd::Stay)), PaneContinuation::Continue(()));
    assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#   >  w     #", "\n{}", curr_pane_state.as_ascii_rows().join("\n")); // R
    assert_eq!(curr_pane_state.advance(&mut Input::from_cmd(Cmd::Stay)), PaneContinuation::Continue(()));
    assert_eq!(&curr_pane_state.as_ascii_rows()[2], "#    > w     #", "\n{}", curr_pane_state.as_ascii_rows().join("\n")); // F
}
