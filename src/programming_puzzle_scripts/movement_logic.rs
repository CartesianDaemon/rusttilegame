// TODO: Add these types to a Script struct?
use crate::engine::scripting::*;
use super::obj_types::*;

pub fn passable(field: &Field, pos: MapCoord) -> bool {
    field.all_pass(pos, Pass::Empty)
}

pub fn impassable(field: &Field, pos: MapCoord) -> bool {
    !passable(field, pos)
}

pub fn move_mov(field: &mut Field, mov: RosterIndex, cmd: Cmd) -> SceneContinuation {
    let hero = field.hero();
    match field[mov].props.ai {
        AI::Hero => {
            // TODO
        },
        AI::Stay => {
            // Do nothing
        },
        AI::Bounce => {
            // ???? TODO: Remove. TODO combine two match branches.
        },
        AI::Drift => {
            // ????
        },
        }
    return SceneContinuation::Continue(());
}
