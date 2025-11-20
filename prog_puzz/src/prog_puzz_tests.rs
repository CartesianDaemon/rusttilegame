use std::collections::HashMap;

use tile_engine::for_gamedata::*;
use super::objs::*;

static INITIALISE_ONCE: std::sync::Once = std::sync::Once::new();

fn initialise() {
    INITIALISE_ONCE.call_once(|| {
        tile_engine::infra::log_builder()
            .filter_level(log::LevelFilter::Trace)
            .is_test(true)
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

fn basic_map(turn: usize) -> Arena<super::game_logic::ProgpuzzGameLogic> {
    let face = match turn {
        0..=2 => '^',
        3..=5 => '>',
        _ => '?',
    };
    let pos = match turn {
        3 => '2',
        0..5 => turn.to_string().chars().nth(0).unwrap(),
        _ => '?',
    };
    let mut ascii = [
        "##############",
        "#            #",
        "#   245w     #",
        "#   1        #",
        "#   0        #",
        "#            #",
        "##############",
    ].map(|row| row.to_string());
    for row in &mut ascii {
        *row = row.chars().map(|char|
            match char {
                '0'..='9' if char==pos => face,
                '0'..='9' => ' ',
                _ => char,
            }
        ).collect::<String>()
    }
    let key = basic_test_key();
    Arena::from_map_and_key(&ascii, key)
}

fn get_basic_lev() -> Widget<super::game_logic::ProgpuzzGameLogic> {
    use Op::*;
    Widget::CodingArena(CodingArena::new::<16>(
        basic_map(0),
        Coding::from_hashmap( &[(F, 1), (L, 1), (R, 1), (x2, 1)] )
    ))
}

#[ignore]
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
    if let Widget::CodingArena(coding_arena)= &mut state {
        use Op::*;
        coding_arena.coding.prog =  Prog::from(vec![F,F,R,F]);
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
