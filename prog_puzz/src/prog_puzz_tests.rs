use std::collections::HashMap;

use macroquad::prelude::*;

use tile_engine::for_gamedata::*;
use super::objs::*;

fn get_lev(n: i32) -> Pane<super::movement_logic::ProgpuzzMovementLogic> {
    // NB: Use progpuzz key directly``
    let prog = Prog::from("F,F,R,F");
    let test_key = HashMap::from([
        (' ', vec![ new_floor() ]),
        ('#', vec![ new_floor(), new_wall() ]),
        ('^', vec![ new_floor(), new_progbot_with_prog(CoordDelta::from_xy(0, -1), &prog) ]),
        ('>', vec![ new_floor(), new_progbot_with_prog(CoordDelta::from_xy(1, 0), &prog) ]),
        ('v', vec![ new_floor(), new_progbot_with_prog(CoordDelta::from_xy(0, 1), &prog) ]),
        ('<', vec![ new_floor(), new_progbot_with_prog(CoordDelta::from_xy(-1, 0), &prog) ]),
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
    // TODO: Move assert line into function. With some way of seeing how many ticks have passed.
    // NB: Get away from as_play. Instead have initial map with 0123 in, and fn to say which is ^, or other mov.
    // Then check that state is exactly the Pane::Something(Something) we expect.
    // Need to have decent visualisation for Pane::Something(Something).
    // Including checking that we move through level transitions ok.

    let mut state = get_lev(1);
    // assert!(matches!(state, Pane::Split(phase: SplitPhase::Running, ..)));
    assert_eq!(state.as_ascii_rows(), get_lev(1).as_ascii_rows());

    // Start running, no other effect
    assert_eq!(state.advance(Some(Cmd::Stay)), PaneContinuation::Continue(()));
    //assert_eq!(state.phase, SplitPhase::Coding);
    assert_eq!(&state.as_ascii_rows()[4], "#   ^        #", "\n{}", state.as_ascii_rows().join("\n"));

    assert_eq!(state.advance(Some(Cmd::Stay)), PaneContinuation::Continue(()));
    //assert_eq!(state.phase, SplitPhase::Running);
    assert_eq!(&state.as_ascii_rows()[3], "#   ^        #", "\n{}", state.as_ascii_rows().join("\n")); // F

    assert_eq!(state.advance(Some(Cmd::Stay)), PaneContinuation::Continue(()));
    assert_eq!(&state.as_ascii_rows()[2], "#   ^  w     #", "\n{}", state.as_ascii_rows().join("\n")); // F

    assert_eq!(state.advance(Some(Cmd::Stay)), PaneContinuation::Continue(()));
    assert_eq!(&state.as_ascii_rows()[2], "#   >  w     #", "\n{}", state.as_ascii_rows().join("\n")); // R

    assert_eq!(state.advance(Some(Cmd::Stay)), PaneContinuation::Continue(()));
    assert_eq!(&state.as_ascii_rows()[2], "#    > w     #", "\n{}", state.as_ascii_rows().join("\n")); // F
}
