// TODO: Add these types to BaseScripts struct??
use crate::engine::for_scripting::*;

type Map = crate::engine::for_scripting::Map<super::super::gamedata_progpuzz::ProgpuzzCustomProps>;

pub fn passable(field: &Map, pos: MapCoord) -> bool {
    field.all_pass(pos, Pass::Empty)
}

#[allow(dead_code)]
pub fn impassable(field: &Map, pos: MapCoord) -> bool {
    !passable(field, pos)
}

pub struct ProgpuzzMovementLogic;

impl BaseMovementLogic for ProgpuzzMovementLogic
{
    type CustomProps = super::super::gamedata_progpuzz::ProgpuzzCustomProps;

    fn move_mov(field: &mut Map<Self>, mov: RosterIndex, cmd: Cmd) -> PaneContinuation {
        match field[mov].logical_props.ai {
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
                    PaneContinuation::Break(PaneEnding::PlayWin)
                } else {
                    PaneContinuation::Continue(())
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
        return PaneContinuation::Continue(());
    }
}