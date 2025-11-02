// TODO: Add these types to a Script struct?
use crate::engine::for_scripting::*;

#[derive(Clone, Debug)]
pub struct ProgpuzzScripts {
}

impl BaseScripts for ProgpuzzScripts {
    fn move_mov(field: &mut Field, mov: RosterIndex, cmd: Cmd) -> SceneContinuation {
        move_mov(field, mov, cmd)
    }
}

impl ProgpuzzScripts {
}

pub fn passable(field: &Field, pos: MapCoord) -> bool {
    field.all_pass(pos, Pass::Empty)
}

#[allow(dead_code)]
pub fn impassable(field: &Field, pos: MapCoord) -> bool {
    !passable(field, pos)
}

pub fn move_mov(field: &mut Field, mov: RosterIndex, cmd: Cmd) -> SceneContinuation {
    match field[mov].props.ai {
        AI::Hero => {
            // TODO make sure cmd makes sense as program instruction not key
            if cmd != Cmd::Stay {
                let target_pos = field[mov].pos() + cmd.as_dir();
                if passable(field, target_pos) {
                    field.move_obj_to(mov, target_pos);
                }
            }
            // Check for goal
            return if field.any_effect(field[mov].pos(), Effect::Win) {
                SceneContinuation::Break(SceneEnding::PlayWin)
            } else {
                SceneContinuation::Continue(())
            }
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
        AI::Scuttle => {
            // ????
        },
        }
    return SceneContinuation::Continue(());
}
