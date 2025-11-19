use std::collections::HashMap;

use tile_engine::for_gamedata::*;
use super::objs::*;

static INITIALISE_ONCE: std::sync::Once = std::sync::Once::new();

fn initialise() {
    INITIALISE_ONCE.call_once(|| {
        tile_engine::infra::log_builder()
            .filter_level(log::LevelFilter::Debug)
            .target(env_logger::Target::Stdout)
            .init();
        log::info!("Initialised logging for tests.");
    });
}

fn basic_test_key() -> HashMap<char, Vec<FreeObj<crate::game_logic::ProgpuzzCustomProps>>> {
    let prog = Prog::from("F,F,R,F");
    HashMap::from([
        (' ', vec![ new_floor() ]),
        ('#', vec![ new_floor(), new_wall() ]),
        ('^', vec![ new_floor(), new_progbot_with_prog(CoordDelta::from_xy(0, -1), &prog) ]),
        ('>', vec![ new_floor(), new_progbot_with_prog(CoordDelta::from_xy(1, 0), &prog) ]),
        ('v', vec![ new_floor(), new_progbot_with_prog(CoordDelta::from_xy(0, 1), &prog) ]),
        ('<', vec![ new_floor(), new_progbot_with_prog(CoordDelta::from_xy(-1, 0), &prog) ]),
        ('w', vec![ new_door_win() ]),
    ])
}

fn get_basic_lev() -> Widget<super::game_logic::ProgpuzzGameLogic> {
    // NB: Use progpuzz key directly``
    Widget::CodingArena(CodingArena::new::<16>(
        Arena::from_ascii(&[
        "##############",
        "#            #",
        "#      w     #",
        "#            #",
        "#   ^        #",
        "#            #",
        "##############",
        ], basic_test_key()),
        Coding::from_ascii(
            // NB: Consider crate macro to initialise vec
            // NB: Consider my iteration macro here and elsewhere I collect'ed.
            [
                ("F", 1),
                ("L", 1),
                ("R", 1),
                ("x2", 1),
            ].into_iter().collect()
        )
    ))
}

//#[ignore]
#[test]
fn basic_move() {
    initialise();

    // TODO: Move assert line into function. With some way of seeing how many ticks have passed.
    // NB: Get away from as_play. Instead have initial map with 0123 in, and fn to say which is ^, or other mov.
    // Then check that state is exactly the Pane::Something(Something) we expect.
    // Need to have decent visualisation for Pane::Something(Something).
    // Including checking that we move through level transitions ok.

    let mut state = get_basic_lev();
    // assert!(matches!(state, Pane::Split(phase: SplitPhase::Running, ..)));
    assert_eq!(state.as_ascii_rows(), get_basic_lev().as_ascii_rows());

    // Set up program that "user" has entered in code pane, that bot will follow.
    if let Widget::CodingArena(split)= &mut state {
        split.coding.prog =  Prog::from("F,F,R,F");
    }

    // Start running, no other effect
    assert_eq!(state.advance(MoveCmd::Stay), PaneContinuation::Continue(()));
    //assert_eq!(state.phase, SplitPhase::Coding);
    assert_eq!(&state.as_ascii_rows()[4], "#   ^        #", "\n{}", state.as_ascii_rows().join("\n"));

    assert_eq!(state.advance(MoveCmd::Stay), PaneContinuation::Continue(()));
    //assert_eq!(state.phase, SplitPhase::Running);
    assert_eq!(&state.as_ascii_rows()[3], "#   ^        #", "\n{}", state.as_ascii_rows().join("\n")); // F

    assert_eq!(state.advance(MoveCmd::Stay), PaneContinuation::Continue(()));
    assert_eq!(&state.as_ascii_rows()[2], "#   ^  w     #", "\n{}", state.as_ascii_rows().join("\n")); // F

    assert_eq!(state.advance(MoveCmd::Stay), PaneContinuation::Continue(()));
    assert_eq!(&state.as_ascii_rows()[2], "#   >  w     #", "\n{}", state.as_ascii_rows().join("\n")); // R

    assert_eq!(state.advance(MoveCmd::Stay), PaneContinuation::Continue(()));
    assert_eq!(&state.as_ascii_rows()[2], "#    > w     #", "\n{}", state.as_ascii_rows().join("\n")); // F
}

#[test]
fn basic_move2() {
    initialise();

    let mut state = get_basic_lev();
    assert!(matches!(state, Widget::CodingArena(CodingArena{phase: CodingRunningPhase::Coding, ..})));
    assert_eq!(state.as_ascii_rows(), get_basic_lev().as_ascii_rows());

    // Set up program that "user" has entered in code pane, that bot will follow.
    if let Widget::CodingArena(split)= &mut state {
        split.coding.prog =  Prog::from("F,F,R,F");
    }

    // Start running, no other effect
    assert_eq!(state.advance(MoveCmd::Stay), PaneContinuation::Continue(()));
    assert!(matches!(state, Widget::CodingArena(CodingArena{phase: CodingRunningPhase::Running, ..})));
    assert_eq!(&state.as_ascii_rows()[4], "#   ^        #", "\n{}", state.as_ascii_rows().join("\n"));

    assert_eq!(state.advance(MoveCmd::Stay), PaneContinuation::Continue(()));
    assert_eq!(&state.as_ascii_rows()[3], "#   ^        #", "\n{}", state.as_ascii_rows().join("\n")); // F

    assert_eq!(state.advance(MoveCmd::Stay), PaneContinuation::Continue(()));
    assert_eq!(&state.as_ascii_rows()[2], "#   ^  w     #", "\n{}", state.as_ascii_rows().join("\n")); // F

    assert_eq!(state.advance(MoveCmd::Stay), PaneContinuation::Continue(()));
    assert_eq!(&state.as_ascii_rows()[2], "#   >  w     #", "\n{}", state.as_ascii_rows().join("\n")); // R

    assert_eq!(state.advance(MoveCmd::Stay), PaneContinuation::Continue(()));
    assert_eq!(&state.as_ascii_rows()[2], "#    > w     #", "\n{}", state.as_ascii_rows().join("\n")); // F
}
