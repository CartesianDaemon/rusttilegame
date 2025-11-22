use std::collections::HashMap;

use tile_engine::for_gamedata::{arena::MapObj, *};
use crate::game_logic::{ProgpuzzCustomProps, ProgpuzzGameLogic};

use super::objs::*;

static INITIALISE_ONCE: std::sync::Once = std::sync::Once::new();

fn initialise() {
    INITIALISE_ONCE.call_once(|| {
        tile_engine::infra::log_builder()
            .filter_level(log::LevelFilter::Debug)
            .is_test(true)
            .init();
        log::info!("Initialised logging for tests.");
    });
}

fn basic_test_key() -> HashMap<char, Vec<FreeObj<crate::game_logic::ProgpuzzCustomProps>>> {
    use Op::*;
    let prog = Prog::from(vec![F,F,R,F]);
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
        Coding::from_vec( &[(F, 1), (L, 1), (R, 1), (group, 1)] )
    ))
}

fn get_basic_lev_with_prog(prog: Prog) -> Widget<super::game_logic::ProgpuzzGameLogic> {
    let mut state = get_basic_lev();
    if let Widget::CodingArena(coding_arena)= &mut state {
        coding_arena.coding.prog =  prog;
    };
    state
}

fn coding_arena<'a>(state: &'a Widget<ProgpuzzGameLogic>) -> &'a CodingArena<ProgpuzzGameLogic> {
    match state {
        Widget::CodingArena(coding_arena) => &coding_arena,
        _ => panic!("Can't find hero. Arena may not be running."),
    }
}

fn hero<'a>(state: &'a Widget<ProgpuzzGameLogic>) -> &'a MapObj<ProgpuzzCustomProps> {
    match state {
        Widget::CodingArena(CodingArena {curr_arena: Some(arena), .. }) => &arena[arena.hero()],
        _ => panic!("Can't find hero. Arena may not be running."),
    }
}

fn hero_prog<'a>(state: &'a Widget<ProgpuzzGameLogic>) -> &'a Prog {
    &hero(state).logical_props.custom_props.prog
}

#[test]
fn basic_move() {
    initialise();

    use Op::*;
    let mut state = get_basic_lev_with_prog(Prog::from(vec![F,F,R,F]));
    assert!(matches!(state, Widget::CodingArena(CodingArena{phase: CodingRunningPhase::Coding, ..})));
    assert_eq!(state.as_ascii_rows(), get_basic_lev().as_ascii_rows());
    assert_eq!(ProgpuzzGameLogic::get_active_idx(coding_arena(&state)), None);

    // Start running, no other effect
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert!(matches!(state, Widget::CodingArena(CodingArena{phase: CodingRunningPhase::Running, ..})));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(4, 4));
    assert!(hero_prog(&state).not_begun());
    assert_eq!(hero_prog(&state).next_op(), F);
    assert_eq!(ProgpuzzGameLogic::get_active_idx(coding_arena(&state)).unwrap(), 0);

    // F
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(4, 3));

    assert_eq!(hero_prog(&state).curr_op(), F);
    assert_eq!(hero_prog(&state).next_op(), F);
    assert_eq!(ProgpuzzGameLogic::get_active_idx(coding_arena(&state)).unwrap(), 0);

    // F
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(4, 2));

    assert_eq!(hero_prog(&state).curr_op(), F);
    assert_eq!(hero_prog(&state).next_op(), R);
    assert_eq!(ProgpuzzGameLogic::get_active_idx(coding_arena(&state)).unwrap(), 1);

    // R
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(4, 2));
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(1, 0));

    assert_eq!(hero_prog(&state).curr_op(), R);
    assert_eq!(hero_prog(&state).next_op(), F);
    assert_eq!(ProgpuzzGameLogic::get_active_idx(coding_arena(&state)).unwrap(), 2);

    // F
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(5, 2));

    assert_eq!(hero_prog(&state).curr_op(), F);
    assert!(hero_prog(&state).has_reached_end());
    assert_eq!(ProgpuzzGameLogic::get_active_idx(coding_arena(&state)).unwrap(), 3);
}

#[test]
fn group() {
    initialise();

    use Op::*;
    let mut prog = Prog::from(vec![R,group,R]);
    prog.instrs[1].subnodes = Some(Prog::from(vec![F, F]));
    let mut state = get_basic_lev_with_prog(prog);

    // Start running, no other effect
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert!(hero_prog(&state).not_begun());
    assert_eq!(hero_prog(&state).next_op(), R);

    assert!(matches!(state, Widget::CodingArena(CodingArena{phase: CodingRunningPhase::Running, ..})));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(4, 4));

    // R
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero_prog(&state).curr_op(), R);
    assert_eq!(hero_prog(&state).next_op(), F);

    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(1, 0));

    // group:F, first F
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(())); // x2 instr unimplemented!()
    assert_eq!(hero_prog(&state).curr_op(), F);
    assert_eq!(hero_prog(&state).next_op(), F);

    assert_eq!(hero(&state).pos(), MapCoord::from_xy(5, 4));

    // group:F, second F
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero_prog(&state).curr_op(), F);
    assert_eq!(hero_prog(&state).next_op(), R);

    assert_eq!(hero(&state).pos(), MapCoord::from_xy(6, 4));

    // R
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero_prog(&state).curr_op(), R);
    assert!(hero_prog(&state).has_reached_end());

    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(0, 1));
}

#[test]
fn repeat_x2() {
    initialise();

    use Op::*;
    let mut prog = Prog::from(vec![R,x2,R]);
    prog.instrs[1].subnodes = Some(Prog::from(vec![F]));
    let mut state = get_basic_lev_with_prog(prog);

    // Start running, no other effect
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert!(matches!(state, Widget::CodingArena(CodingArena{phase: CodingRunningPhase::Running, ..})));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(4, 4));

    // R
    assert_eq!(hero_prog(&state).next_op(), R);
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(1, 0));

    assert_eq!(hero_prog(&state).curr_op(), R);

    // x2 F, first time
    assert_eq!(hero_prog(&state).next_op(), F);
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(5, 4));
    assert_eq!(hero_prog(&state).curr_op(), F);

    // x2 F, second time
    assert_eq!(hero_prog(&state).next_op(), F);
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(6, 4));
    assert_eq!(hero_prog(&state).curr_op(), F);

    // R
    assert_eq!(hero_prog(&state).next_op(), R);
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(0, 1));
    assert_eq!(hero_prog(&state).curr_op(), R);

    assert!(hero_prog(&state).has_reached_end());
}

#[test]
fn repeat_x2_rotate() {
    initialise();

    use Op::*;
    let mut prog = Prog::from(vec![x2]);
    prog.instrs[0].subnodes = Some(Prog::from(vec![R,R,L]));
    let mut state = get_basic_lev_with_prog(prog);

    // Start running, no other effect
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(0, -1));

    // R
    assert_eq!(hero_prog(&state).next_op(), R);
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(1, 0));

    // R
    assert_eq!(hero_prog(&state).next_op(), R);
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(0, 1));

    // L
    assert_eq!(hero_prog(&state).next_op(), L);
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(1, 0));

    if true {
        // R
        assert_eq!(hero_prog(&state).next_op(), R);
        assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
        assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(0, 1));

        // R
        // assert_eq!(hero_prog(&state).next_op(), R);
        assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
        assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(-1, 0));

        // L
        // assert_eq!(hero_prog(&state).next_op(), L);
        assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
        assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(0, 1));

        assert!(hero_prog(&state).has_reached_end());
    }
}

#[test]
fn nested_repeat() {
    initialise();

    use Op::*;
    let mut prog = Prog::from(vec![R, x2]);
    prog.instrs[1].subnodes = Some(Prog::from(vec![x2]));
    prog.instrs[1].subnodes.as_mut().unwrap().instrs[0].subnodes = Some(Prog::from(vec![F]));
    let mut state = get_basic_lev_with_prog(prog);

    // Start running, no other effect
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(4, 4));
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(0, -1));

    // R
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(1, 0));

    // F
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(5, 4));

    // F
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(6, 4));

    // F
    assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(7, 4));

    if false {
        // F
        assert_eq!(state.advance(MoveCmd::Stay), WidgetContinuation::Continue(()));
        assert_eq!(hero(&state).pos(), MapCoord::from_xy(8, 4));
    }

    assert!(hero_prog(&state).has_reached_end());
}
