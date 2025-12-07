use std::collections::HashMap;

use tile_engine::for_gamedata::{arena::MapObj, *};
use tile_engine::infra::initialise_logging_for_tests;
use crate::game_logic::{ProgpuzzCustomProps, ProgpuzzGameLogic};

use super::objs::*;

fn basic_test_key() -> HashMap<char, Vec<FreeObj<crate::game_logic::ProgpuzzCustomProps>>> {
    use prog_ops::*;
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

fn get_basic_lev() -> Scene<super::game_logic::ProgpuzzGameLogic> {
    use supply_ops::*;
    Scene::CodingArena(CodingArena::new::<16>(
        basic_map(0),
        Coding::from_vec( &[(F, 1), (L, 1), (R, 1), (group, 1)] )
    ))
}

fn get_basic_lev_with_prog(prog: Prog) -> Scene<super::game_logic::ProgpuzzGameLogic> {
    let mut state = get_basic_lev();
    if let Scene::CodingArena(coding_arena)= &mut state {
        coding_arena.coding.prog =  prog;
    };
    state
}

fn coding_arena<'a>(state: &'a Scene<ProgpuzzGameLogic>) -> &'a CodingArena<ProgpuzzGameLogic> {
    match state {
        Scene::CodingArena(coding_arena) => &coding_arena,
        _ => panic!("Can't find hero. Arena may not be running."),
    }
}

fn hero<'a>(state: &'a Scene<ProgpuzzGameLogic>) -> &'a MapObj<ProgpuzzCustomProps> {
    match state {
        Scene::CodingArena(CodingArena {curr_arena: Some(arena), .. }) => &arena[arena.hero()],
        _ => panic!("Can't find hero. Arena may not be running."),
    }
}

fn hero_prog<'a>(state: &'a Scene<ProgpuzzGameLogic>) -> &'a Prog {
    &hero(state).logical_props.custom_props.prog
}

#[test]
fn basic_move() {
    initialise_logging_for_tests();

    use prog_ops::*;
    let mut state = get_basic_lev_with_prog(Prog::from(vec![F,F,R,F]));
    assert!(matches!(state, Scene::CodingArena(CodingArena{phase: CodingRunningPhase::Coding, ..})));
    assert_eq!(state.as_ascii_rows(), get_basic_lev().as_ascii_rows());
    assert_eq!(ProgpuzzGameLogic::get_active_idx(coding_arena(&state)), None);

    // Start running, no other effect
    state.advance(InputCmd::NextPhase); assert_eq!(state.ready_for_next_level(), None); 
    assert!(matches!(state, Scene::CodingArena(CodingArena{phase: CodingRunningPhase::Running, ..})));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(4, 4));
    assert_eq!(ProgpuzzGameLogic::get_active_idx(coding_arena(&state)).unwrap(), 0);

    // F
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(ProgpuzzGameLogic::get_active_idx(coding_arena(&state)).unwrap(), 0);
    assert_eq!(hero_prog(&state).unwrap_curr_op(), &F);
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(4, 3));

    // F
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(ProgpuzzGameLogic::get_active_idx(coding_arena(&state)).unwrap(), 1);
    assert_eq!(hero_prog(&state).unwrap_curr_op(), &F);
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(4, 2));

    // R
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(ProgpuzzGameLogic::get_active_idx(coding_arena(&state)).unwrap(), 2);
    assert_eq!(hero_prog(&state).unwrap_curr_op(), &R);
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(4, 2));
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(1, 0));

    // F
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(ProgpuzzGameLogic::get_active_idx(coding_arena(&state)).unwrap(), 3);
    assert_eq!(hero_prog(&state).unwrap_curr_op(), &F);
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(5, 2));
}

#[test]
fn test_group() {
    initialise_logging_for_tests();

    use prog_ops::*;
    let mut prog = Prog::from(vec![R,group,R]);
    prog[1] = Instr::Parent(ParentOpcode::group, Prog::from("F, F"));
    let mut state = get_basic_lev_with_prog(prog);

    // Start running, no other effect
    state.advance(InputCmd::NextPhase); assert_eq!(state.ready_for_next_level(), None); 

    assert!(matches!(state, Scene::CodingArena(CodingArena{phase: CodingRunningPhase::Running, ..})));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(4, 4));

    // R
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero_prog(&state).unwrap_curr_op(), &R);

    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(1, 0));

    // group:F, first F
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None); // x2 instr unimplemented!()
    assert_eq!(hero_prog(&state).unwrap_curr_op(), &F);
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(5, 4));

    // group:F, second F
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero_prog(&state).unwrap_curr_op(), &F);

    assert_eq!(hero(&state).pos(), MapCoord::from_xy(6, 4));

    // R
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero_prog(&state).unwrap_curr_op(), &R);

    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(0, 1));
}

#[test]
fn repeat_x2() {
    initialise_logging_for_tests();

    use prog_ops::*;
    let mut prog = Prog::from(vec![R,x2,R]);
    prog[1] = Instr::Parent(ParentOpcode::x2, Prog::from("F"));
    let mut state = get_basic_lev_with_prog(prog);

    // Start running, no other effect
    state.advance(InputCmd::NextPhase); assert_eq!(state.ready_for_next_level(), None);
    assert!(matches!(state, Scene::CodingArena(CodingArena{phase: CodingRunningPhase::Running, ..})));
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(4, 4));

    // R
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(1, 0));

    assert_eq!(hero_prog(&state).unwrap_curr_op(), &R);

    // x2 F, first time
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(5, 4));
    assert_eq!(hero_prog(&state).unwrap_curr_op(), &F);

    // x2 F, second time
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(6, 4));
    assert_eq!(hero_prog(&state).unwrap_curr_op(), &F);

    // R
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(0, 1));
    assert_eq!(hero_prog(&state).unwrap_curr_op(), &R);
}

#[test]
fn repeat_x2_rotate() {
    initialise_logging_for_tests();

    use prog_ops::*;
    let mut prog = Prog::from(vec![x2]);
    prog[0] = Instr::Parent(ParentOpcode::x2, Prog::from("R, R, L"));
    let mut state = get_basic_lev_with_prog(prog);

    // Start running, no other effect
    state.advance(InputCmd::NextPhase); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(0, -1));

    // R
    assert_eq!(hero_prog(&state).unwrap_curr_op(), &R);
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(1, 0));

    // R
    assert_eq!(hero_prog(&state).unwrap_curr_op(), &R);
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(0, 1));

    // L
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero_prog(&state).unwrap_curr_op(), &L);
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(1, 0));

    // R
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero_prog(&state).unwrap_curr_op(), &R);
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(0, 1));

    // R
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    // assert_eq!(hero_prog(&state).unwrap_curr_op(), &R);
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(-1, 0));

    // L
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    // assert_eq!(hero_prog(&state).unwrap_curr_op(), &L);
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(0, 1));
}

#[test]
fn nested_repeat() {
    initialise_logging_for_tests();

    use prog_ops::*;
    let mut prog = Prog::from(vec![R, x2]);
    prog[1] = Instr::Parent(ParentOpcode::x2, Prog::from("x2"));
    prog[1][0] = Instr::Parent(ParentOpcode::x2, Prog::from("F"));
    let mut state = get_basic_lev_with_prog(prog);

    // Start running, no other effect
    state.advance(InputCmd::NextPhase); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(4, 4));
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(0, -1));

    // R
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero(&state).logical_props.dir, CoordDelta::from_xy(1, 0));

    // F
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(5, 4));

    // F
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(6, 4));

    // F
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(7, 4));

    // F
    state.advance(InputCmd::Tick); assert_eq!(state.ready_for_next_level(), None);
    assert_eq!(hero(&state).pos(), MapCoord::from_xy(8, 4));
}
